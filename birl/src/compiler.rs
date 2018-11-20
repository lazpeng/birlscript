use std::collections::HashMap;
use parser::{ Expression, ExpressionNode, FunctionParameter, Command, TypeKind, CommandArgument, MathOperator, CommandKind };
use vm::{ Instruction, ComparisionRequest };
use context::RawValue;

#[derive(Debug)]
enum SubScopeKind {
    Loop,
    ExecuteIf,
    Regular,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ScopeKind {
    Function,
    Global
}

#[derive(Debug, Clone)]
struct SymbolEntry {
    address : usize,
    global : bool,
    writeable : bool,
}

impl SymbolEntry {
    fn from(address : usize, global : bool, writeable : bool) -> SymbolEntry {
        SymbolEntry { address, global, writeable }
    }
}

#[derive(Debug)]
struct ScopeInfo {
    symbol_table : HashMap<String, SymbolEntry>,
    scope_kind : SubScopeKind,
    previous_next_var_address : usize,
}

impl ScopeInfo {
    fn new(scope_kind : SubScopeKind, previous_next_var_address : usize, is_global : bool) -> ScopeInfo {
        let mut symbol_table = HashMap::new();
        symbol_table.insert("TREZE".to_owned(), SymbolEntry::from(0, is_global, false));

        ScopeInfo {
            symbol_table,
            scope_kind,
            previous_next_var_address,
        }
    }
}

struct FunctionInfo {
    address : usize,
    arguments : Vec<TypeKind>,
}

impl FunctionInfo {
    fn from(address : usize, arguments : Vec<TypeKind>) -> FunctionInfo {
        FunctionInfo { address, arguments }
    }
}

pub enum CompilerHint {
    ScopeStart,
    ScopeEnd,
}

pub struct Compiler {
    scopes : Vec<ScopeInfo>,
    functions : HashMap<String, FunctionInfo>,
    next_var_address : usize,
    current_scope : ScopeKind,
}

impl Compiler {
    pub fn new() -> Compiler {
        let mut funcs = HashMap::new();
        funcs.insert("__global__".to_owned(), FunctionInfo::from(0, vec![]));
        funcs.insert("SHOW".to_owned(), FunctionInfo::from(1, vec![]));

        Compiler {
            scopes : vec![ScopeInfo::new(SubScopeKind::Regular, 1, true)],
            functions : funcs,
            next_var_address : 1,
            current_scope : ScopeKind::Global,
        }
    }

    fn get_inst_for_op(op : MathOperator) -> Option<Instruction> {
        match op {
            MathOperator::Plus => Some(Instruction::Add),
            MathOperator::Minus => Some(Instruction::Sub),
            MathOperator::Division => Some(Instruction::Div),
            MathOperator::Multiplication => Some(Instruction::Mul),
            _ => None,
        }
    }

    pub fn compile_expression(&self, expr : Expression, inst : &mut Vec<Instruction>) -> Result<(), String> {

        inst.push(Instruction::SetFirstExpressionOperation);

        let mut is_a = expr.nodes.len() > 1;

        for node in expr.nodes {
            match node {
                ExpressionNode::Operator(MathOperator::ParenthesisLeft) |
                ExpressionNode::Operator(MathOperator::ParenthesisRight) => unreachable!(),
                ExpressionNode::Operator(o) => {
                    let opi = match Compiler::get_inst_for_op(o) {
                        Some(i) => i,
                        None => unreachable!(),
                    };

                    inst.push(opi);

                    is_a = true;
                }
                ExpressionNode::Value(raw) => {
                    if is_a {
                        inst.push(Instruction::PushValMathA(raw));
                    } else {
                        inst.push(Instruction::PushValMathB(raw));
                    }

                    is_a = !is_a;
                }
                ExpressionNode::Symbol(s) => {
                    let info = match self.find_symbol(s.as_str()) {
                        Some(i) => i,
                        None => return Err(format!("Variável não encontrada : {}", s)),
                    };

                    if info.global {
                        inst.push(Instruction::ReadGlobalVarFrom(info.address));
                    } else {
                        inst.push(Instruction::ReadVarFrom(info.address));
                    }

                    if is_a {
                        inst.push(Instruction::PushIntermediateToA);
                    } else {
                        inst.push(Instruction::PushIntermediateToB);
                    }

                    is_a = !is_a;
                }
            }
        }

        Ok(())
    }

    fn end_scope(&mut self, info : ScopeInfo) {
        self.next_var_address = info.previous_next_var_address;
    }

    fn find_symbol(&self, name : &str) -> Option<&SymbolEntry> {
        for scope in (&self.scopes).into_iter().rev() {
            match scope.symbol_table.get(name) {
                Some(v) => return Some(v),
                None => {}
            }
        }

        None
    }

    fn add_symbol(&mut self, name : String, writeable : bool) -> Option<SymbolEntry> {
        let is_global = self.current_scope == ScopeKind::Global;
        let entry = SymbolEntry::from(self.next_var_address, is_global, writeable);

        match self.scopes.last_mut() {
            Some(s) => {
                s.symbol_table.insert(name, entry.clone());
                Some(entry)
            }
            None => None,
        }
    }

    fn find_or_add_symbol(&mut self, name : &str, writeable : bool) -> Option<SymbolEntry> {
        if self.scopes.is_empty() {
            None
        } else {
            match self.find_symbol(name) {
                Some(s) => return Some(s.clone()),
                None => {},
            }

            match self.add_symbol(name.to_owned(), writeable) {
                Some(s) => Some(s),
                None => None,
            }
        }
    }

    fn get_function_info(&self, id : usize) -> Option<&FunctionInfo> {
        for (_, f) in &self.functions {
            if f.address == id {
                return Some(f);
            }
        }

        None
    }

    fn add_execute_while_boilerplate(&self, mut cmd : Command, instructions : &mut Vec<Instruction>) -> Result<(), String> {
        instructions.push(Instruction::AddLoopLabel);

        if let CommandArgument::Expression(expr) = cmd.arguments.remove(0) {
            self.compile_expression(expr, instructions)?;
        } else {
            return Err("Argumento 1 não é expressão".to_owned());
        }

        // Move result to A
        instructions.push(Instruction::SwapMath);

        if let CommandArgument::Expression(expr) = cmd.arguments.remove(0) {
            self.compile_expression(expr, instructions)?;
        } else {
            return Err("Argumento 2 não é expressão".to_owned());
        }

        instructions.push(Instruction::Compare);

        Ok(())
    }

    pub fn compile_command(&mut self, mut cmd : Command, instructions : &mut Vec<Instruction>)
            -> Result<Option<CompilerHint>, String> {
        match cmd.kind {
            CommandKind::PrintDebug => {
                // Evaluate the single argument and print-debug it

                if cmd.arguments.len() != 1 {
                    return Err("Internal error : Debug print command has more than 1 argument (or less)".to_owned());
                }

                for arg in cmd.arguments {
                    match arg {
                        CommandArgument::Expression(expr) => {
                            match self.compile_expression(expr, instructions) {
                                Ok(_) => {},
                                Err(e) => return Err(e),
                            };

                            instructions.push(Instruction::PrintMathBDebug);
                        }
                        _ => return Err("Erro : Um argumento diferente de valor foi passado pra print. Erro interno.".to_owned()),
                    }
                }
            }
            CommandKind::Print => {
                for arg in cmd.arguments {
                    match arg {
                        CommandArgument::Expression(expr) => {
                            match self.compile_expression(expr, instructions) {
                                Ok(_) => {},
                                Err(e) => return Err(e),
                            };

                            instructions.push(Instruction::PrintMathB);
                        }
                        _ => return Err("Erro : Um argumento diferente de valor foi passado pra print. Erro interno.".to_owned()),
                    }
                }

                instructions.push(Instruction::FlushStdout);
            }
            CommandKind::PrintLn => {
                for arg in cmd.arguments {
                    match arg {
                        CommandArgument::Expression(expr) => {
                            match self.compile_expression(expr, instructions) {
                                Ok(_) => {},
                                Err(e) => return Err(e),
                            };

                            instructions.push(Instruction::PrintMathB);
                        }
                        _ => return Err("Erro : Um argumento diferente de valor foi passado pra print. Erro interno.".to_owned()),
                    }
                }

                instructions.push(Instruction::PrintNewLine);
            }
            CommandKind::Quit => instructions.push(Instruction::Quit),
            CommandKind::Set => {
                if cmd.arguments.len() != 2 {
                    return Err(format!("O comando BORA espera 2 argumentos, mas {} foram passados (Erro interno)", cmd.arguments.len()));
                }

                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(n) => n,
                    _ => return Err(format!("Erro interno : Esperado um nome pro BORA, encontrado {:?}", name_arg)),
                };

                let entry = match self.find_symbol(name.as_str()) {
                    Some(e) => e,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                if ! entry.writeable {
                    return Err(format!("Erro : A variável {} não pode ser escrita", name));
                }

                let expr_arg = cmd.arguments.remove(0);

                match expr_arg {
                    CommandArgument::Expression(expr) => {
                        match self.compile_expression(expr, instructions) {
                            Ok(_) => {}
                            Err(e) => return Err(e)
                        }
                    }
                    _ => return Err(format!("Erro interno : Esperado uma expressão depois do nome, encontrado {:?}", expr_arg)),
                }

                let inst = if entry.global {
                    Instruction::WriteGlobalVarTo(entry.address)
                } else {
                    Instruction::WriteVarTo(entry.address)
                };

                instructions.push(inst);
            }
            CommandKind::Declare => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(n) => n,
                    _ => return Err(format!("Erro interno : Esperado um nome pro BORA, encontrado {:?}", name_arg)),
                };

                let is_global = self.current_scope == ScopeKind::Global;

                if cmd.arguments.is_empty() {
                    // Set value to Null
                    // To achieve this, we set both Maths to null, then copy B to the var address

                    instructions.push(Instruction::ClearMath);
                } else {
                    let expr_arg = cmd.arguments.remove(0);

                    match expr_arg {
                        CommandArgument::Expression(expr) => {
                            match self.compile_expression(expr, instructions) {
                                Ok(_) => {}
                                Err(e) => return Err(e)
                            }
                        }
                        _ => return Err(format!("Erro interno : Esperado uma expressão depois do nome, encontrado {:?}", expr_arg)),
                    }
                }

                // Add the variable after the expression is parsed, so we can't use the variable before a value is set

                let address = self.next_var_address;
                self.next_var_address += 1;

                match self.scopes.last_mut() {
                    Some(s) => s.symbol_table.insert(name, SymbolEntry::from(address, is_global, true)),
                    None => return Err(format!("Scopes é vazio"))
                };

                if is_global {
                    instructions.push(Instruction::WriteGlobalVarTo(address));
                } else {
                    instructions.push(Instruction::WriteVarTo(address));
                }
            }
            CommandKind::Return => {
                if cmd.arguments.is_empty() {
                    instructions.push(Instruction::ClearMath);
                } else {
                    let expr_arg = cmd.arguments.remove(0);

                    match expr_arg {
                        CommandArgument::Expression(expr) => {
                            match self.compile_expression(expr, instructions) {
                                Ok(_) => {}
                                Err(e) => return Err(e)
                            }
                        }
                        _ => return Err(format!("Esperado uma expressão como argumento pro comando Return, encontrado {:?}", expr_arg)),
                    }
                }

                instructions.push(Instruction::Return);
            }
            CommandKind::Compare => {
                let left_expr_arg = cmd.arguments.remove(0);

                match left_expr_arg {
                    CommandArgument::Expression(expr) => {
                        match self.compile_expression(expr, instructions) {
                            Ok(_) => {}
                            Err(e) => return Err(e)
                        }
                    }
                    _ => return Err(format!("Esperado uma expressão como argumento pro comando Return, encontrado {:?}", left_expr_arg)),
                }

                instructions.push(Instruction::SwapMath);

                let right_expr_arg = cmd.arguments.remove(0);

                match right_expr_arg {
                    CommandArgument::Expression(expr) => {
                        match self.compile_expression(expr, instructions) {
                            Ok(_) => {}
                            Err(e) => return Err(e)
                        }
                    }
                    _ => return Err(format!("Esperado uma expressão como argumento pro comando Return, encontrado {:?}", right_expr_arg)),
                }

                instructions.push(Instruction::Compare);
            }
            CommandKind::EndSubScope => {
                let scope_info = match self.scopes.pop() {
                    Some(s) => s,
                    None => return Err(format!("FIM fora de qualquer scope"))
                };

                match scope_info.scope_kind {
                    SubScopeKind::ExecuteIf => instructions.push(Instruction::EndConditionalBlock),
                    SubScopeKind::Loop => {
                        instructions.push(Instruction::RestoreLoopLabel);
                        instructions.push(Instruction::EndConditionalBlock);
                        instructions.push(Instruction::PopLoopLabel);
                    }
                    SubScopeKind::Regular => {
                        self.scopes.push(scope_info);

                        return Err("Erro : Usando FIM pra finalizar uma função".to_owned());
                    }
                }

                self.end_scope(scope_info);

                return Ok(Some(CompilerHint::ScopeEnd));
            },
            CommandKind::ExecuteIfEqual => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::ExecuteIf,
                                                self.next_var_address, is_global));
                instructions.push(Instruction::ExecuteIf(ComparisionRequest::Equal));

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfNotEqual => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::ExecuteIf,
                                                self.next_var_address, is_global));
                instructions.push(Instruction::ExecuteIf(ComparisionRequest::NotEqual));

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfEqualOrGreater => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::ExecuteIf,
                                                self.next_var_address, is_global));
                instructions.push(Instruction::ExecuteIf(ComparisionRequest::MoreOrEqual));

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfGreater => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::ExecuteIf,
                                                self.next_var_address, is_global));
                instructions.push(Instruction::ExecuteIf(ComparisionRequest::More));

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfEqualOrLess => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::ExecuteIf,
                                                self.next_var_address, is_global));
                instructions.push(Instruction::ExecuteIf(ComparisionRequest::LessOrEqual));

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfLess => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::ExecuteIf,
                                                self.next_var_address, is_global));
                instructions.push(Instruction::ExecuteIf(ComparisionRequest::Less));

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::Call => {
                // First argument is the function name

                let mut is_first = true;
                let mut index = 0usize;
                let mut function_info : Option<&FunctionInfo> = None;
                let num_args = cmd.arguments.len() - 1;

                for arg in cmd.arguments {
                    if is_first {
                        is_first = false;

                        let name = match arg {
                            CommandArgument::Name(n) => n,
                            _ => return Err(format!("Erro interno : Esperado um nome pra função")),
                        };

                        let info = match self.functions.get(name.as_str()) {
                            Some(f) => f,
                            None => return Err(format!("Nenhuma função declarada com nome {}", name))
                        };

                        if info.arguments.len() != num_args {
                            return Err(format!("A função {} espera {} argumentos, mas {} foram passados", name,
                                info.arguments.len(), num_args));
                        }

                        function_info = Some(info);

                        instructions.push(Instruction::MakeNewFrame(info.address));
                    } else {
                        let info = function_info.unwrap();

                        let expr = match arg {
                            CommandArgument::Expression(e) => e,
                            _ => return Err("Erro interno : Era esperado um valor como argumento \
                                                    pro comando.".to_owned()),
                        };

                        let expected_type = info.arguments[index];

                        // The parameter address is, in this case, index + 1 (because the address 0 is reserved to
                        // the return value)

                        match self.compile_expression(expr, instructions) {
                            Ok(_) => {}
                            Err(e) => return Err(e)
                        };

                        instructions.push(Instruction::AssertMathBCompatible(expected_type));

                        instructions.push(Instruction::WriteVarToLast(index + 1));

                        index += 1;
                    }
                }

                instructions.push(Instruction::SetLastFrameReady);
            }
            CommandKind::GetStringInput => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let entry = match self.find_or_add_symbol(name.as_str(), true) {
                    Some(e) => e,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                instructions.push(Instruction::ReadInput);
                instructions.push(Instruction::PushIntermediateToB);

                if entry.global {
                    instructions.push(Instruction::WriteGlobalVarTo(entry.address));
                } else {
                    instructions.push(Instruction::WriteVarTo(entry.address));
                }
            }
            CommandKind::GetIntegerInput => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let entry = match self.find_or_add_symbol(name.as_str(), true) {
                    Some(e) => e,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                instructions.push(Instruction::ReadInput);

                instructions.push(Instruction::PushIntermediateToB);

                instructions.push(Instruction::ConvertToInt);

                if entry.global {
                    instructions.push(Instruction::WriteGlobalVarTo(entry.address));
                } else {
                    instructions.push(Instruction::WriteVarTo(entry.address));
                }
            }
            CommandKind::GetNumberInput => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let entry = match self.find_or_add_symbol(name.as_str(), true) {
                    Some(e) => e,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                instructions.push(Instruction::ReadInput);

                instructions.push(Instruction::PushIntermediateToB);

                instructions.push(Instruction::ConvertToNum);

                if entry.global {
                    instructions.push(Instruction::WriteGlobalVarTo(entry.address));
                } else {
                    instructions.push(Instruction::WriteVarTo(entry.address));
                }
            }
            CommandKind::ConvertToInt => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let entry = match self.find_symbol(name.as_str()) {
                    Some(e) => e,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                if entry.global {
                    instructions.push(Instruction::ReadGlobalVarFrom(entry.address));
                } else {
                    instructions.push(Instruction::ReadVarFrom(entry.address));
                }

                instructions.push(Instruction::PushIntermediateToB);

                instructions.push(Instruction::ConvertToInt);

                if entry.global {
                    instructions.push(Instruction::WriteGlobalVarTo(entry.address));
                } else {
                    instructions.push(Instruction::WriteVarTo(entry.address));
                }
            }
            CommandKind::ConvertToNum => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let entry = match self.find_symbol(name.as_str()) {
                    Some(e) => e,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                if entry.global {
                    instructions.push(Instruction::ReadGlobalVarFrom(entry.address));
                } else {
                    instructions.push(Instruction::ReadVarFrom(entry.address));
                }

                instructions.push(Instruction::PushIntermediateToB);

                instructions.push(Instruction::ConvertToNum);

                if entry.global {
                    instructions.push(Instruction::WriteGlobalVarTo(entry.address));
                } else {
                    instructions.push(Instruction::WriteVarTo(entry.address));
                }
            }
            CommandKind::IntoString => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let entry = match self.find_symbol(name.as_str()) {
                    Some(e) => e,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                if entry.global {
                    instructions.push(Instruction::ReadGlobalVarFrom(entry.address));
                } else {
                    instructions.push(Instruction::ReadVarFrom(entry.address));
                }

                instructions.push(Instruction::PushIntermediateToB);

                instructions.push(Instruction::ConvertToString);

                if entry.global {
                    instructions.push(Instruction::WriteGlobalVarTo(entry.address));
                } else {
                    instructions.push(Instruction::WriteVarTo(entry.address));
                }
            }
            CommandKind::ExecuteWhileEqual => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::Loop, self.next_var_address, is_global));

                self.add_execute_while_boilerplate(cmd, instructions)?;

                instructions.push(Instruction::ExecuteIf(ComparisionRequest::Equal));
                return Ok(Some(CompilerHint::ScopeStart));
            }
            CommandKind::ExecuteWhileNotEqual => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::Loop, self.next_var_address, is_global));

                self.add_execute_while_boilerplate(cmd, instructions)?;

                instructions.push(Instruction::ExecuteIf(ComparisionRequest::NotEqual));
                return Ok(Some(CompilerHint::ScopeStart));
            }
            CommandKind::ExecuteWhileGreater => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::Loop, self.next_var_address, is_global));

                self.add_execute_while_boilerplate(cmd, instructions)?;

                instructions.push(Instruction::ExecuteIf(ComparisionRequest::More));
                return Ok(Some(CompilerHint::ScopeStart));
            }
            CommandKind::ExecuteWhileEqualOrGreater => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::Loop, self.next_var_address, is_global));

                self.add_execute_while_boilerplate(cmd, instructions)?;

                instructions.push(Instruction::ExecuteIf(ComparisionRequest::MoreOrEqual));
                return Ok(Some(CompilerHint::ScopeStart));
            }
            CommandKind::ExecuteWhileLess => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::Loop, self.next_var_address, is_global));

                self.add_execute_while_boilerplate(cmd, instructions)?;

                instructions.push(Instruction::ExecuteIf(ComparisionRequest::Less));
                return Ok(Some(CompilerHint::ScopeStart));
            }
            CommandKind::ExecuteWhileEqualOrLess => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::Loop, self.next_var_address, is_global));

                self.add_execute_while_boilerplate(cmd, instructions)?;

                instructions.push(Instruction::ExecuteIf(ComparisionRequest::LessOrEqual));
                return Ok(Some(CompilerHint::ScopeStart));
            }
            CommandKind::RangeLoop => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::Loop, self.next_var_address, is_global));

                let name = if let CommandArgument::Name(n) = cmd.arguments.remove(0) {
                    n
                } else {
                    return Err("Esperado uma variável pro primeiro argumento do loop".to_owned());
                };

                let entry = match self.find_or_add_symbol(name.as_str(), true) {
                    Some(e) => e,
                    None => return Err(format!("Não foi possível adicionar nem encontrar a variável {}", name)),
                };

                // Initialize counter

                if let CommandArgument::Expression(expr) = cmd.arguments.remove(0) {
                    self.compile_expression(expr, instructions)?;

                    if entry.global {
                        instructions.push(Instruction::WriteGlobalVarTo(entry.address));
                    } else {
                        instructions.push(Instruction::WriteVarTo(entry.address));
                    }
                } else {
                    return Err("Era esperado uma expressão pro segundo argumento de RangedLoop".to_owned());
                }

                let final_expr = if let CommandArgument::Expression(expr) = cmd.arguments.remove(0) {
                    expr
                } else {
                    return Err("Esperado um valor final".to_owned());
                };

                // Register increment procedure

                if cmd.arguments.is_empty() {
                    instructions.push(Instruction::PushValMathB(RawValue::Integer(1)));
                } else {
                    if let CommandArgument::Expression(step_expr) = cmd.arguments.remove(0) {
                        self.compile_expression(step_expr, instructions)?;
                    } else {
                        instructions.push(Instruction::PushValMathB(RawValue::Integer(1)));
                    }
                }

                // Loop starts here

                instructions.push(Instruction::AddLoopLabel);

                instructions.push(Instruction::RegisterIncrementOnRestore(entry.address));

                // Check if should continue

                self.compile_expression(final_expr, instructions)?;

                if entry.global {
                    instructions.push(Instruction::ReadGlobalVarFrom(entry.address));
                } else {
                    instructions.push(Instruction::ReadVarFrom(entry.address));
                }

                instructions.push(Instruction::PushIntermediateToA);

                instructions.push(Instruction::Compare);

                instructions.push(Instruction::ExecuteIf(ComparisionRequest::NotEqual));

                return Ok(Some(CompilerHint::ScopeStart));
            }
            CommandKind::MakeNewList => {
                let name = if let CommandArgument::Name(name) = cmd.arguments.remove(0) {
                    name
                } else {
                    return Err("MakeNewList : Esperado um nome".to_owned());
                };

                let entry = match self.find_or_add_symbol(name.as_str(), true) {
                    Some(a) => a,
                    None => return Err(format!("Não foi possível declarar a variável pra lista {}", name))
                };

                instructions.push(Instruction::MakeNewList);

                if entry.global {
                    instructions.push(Instruction::WriteGlobalVarTo(entry.address));
                } else {
                    instructions.push(Instruction::WriteVarTo(entry.address));
                }
            }
            CommandKind::QueryListSize => {
                let list_name = if let CommandArgument::Name(name) = cmd.arguments.remove(0) {
                    name
                } else {
                    return Err("MakeNewList : Esperado um nome".to_owned());
                };

                let dest_name = if let CommandArgument::Name(name) = cmd.arguments.remove(0) {
                    name
                } else {
                    return Err("MakeNewList : Esperado um nome".to_owned());
                };

                let dest = match self.find_or_add_symbol(dest_name.as_str(), true) {
                    Some(d) => d,
                    None => return Err(format!("Não foi possível declarar a variável pra lista {}", dest_name))
                };

                let list = match self.find_symbol(list_name.as_str()) {
                    Some(a) => a,
                    None => return Err(format!("Não foi possível encontrar a lista {}", list_name))
                };

                if list.global {
                    instructions.push(Instruction::ReadGlobalVarFrom(list.address));
                } else {
                    instructions.push(Instruction::ReadVarFrom(list.address));
                }

                instructions.push(Instruction::QueryListSize);

                if dest.global {
                    instructions.push(Instruction::WriteGlobalVarTo(dest.address));
                } else {
                    instructions.push(Instruction::WriteVarTo(dest.address));
                }
            }
            CommandKind::AddListElement => {
                let list_name = if let CommandArgument::Name(name) = cmd.arguments.remove(0) {
                    name
                } else {
                    return Err("AddListElement : Esperado um nome".to_owned())
                };

                let element = if let CommandArgument::Expression(expr) = cmd.arguments.remove(0) {
                    expr
                } else {
                    return Err("AddListElement : Esperado um elemento".to_owned())
                };

                let index = if cmd.arguments.is_empty() {
                    None
                } else {
                    if let CommandArgument::Expression(expr) = cmd.arguments.remove(0) {
                        Some(expr)
                    } else {
                        return Err("AddListElement : Era esperado uma expressão como um index".to_owned());
                    }
                };

                let list = match self.find_symbol(list_name.as_str()) {
                    Some(l) => l,
                    None => return Err(format!("Não foi possível encontrar a lista {}", list_name))
                };

                if list.global {
                    instructions.push(Instruction::ReadGlobalVarFrom(list.address));
                } else {
                    instructions.push(Instruction::ReadVarFrom(list.address));
                }

                if let Some(expr) = index {
                    self.compile_expression(expr, instructions)?;

                    instructions.push(Instruction::PushMathBToSeconday);
                } else {
                    instructions.push(Instruction::ClearSecondary);
                }

                self.compile_expression(element, instructions)?;

                instructions.push(Instruction::AddToListAtIndex);
            }
            CommandKind::RemoveListElement => {
                let name = if let CommandArgument::Name(name) = cmd.arguments.remove(0) {
                    name
                } else {
                    return Err("RemoveListElement : Esperado um nome".to_owned())
                };

                let index = if let CommandArgument::Expression(expr) = cmd.arguments.remove(0) {
                    expr
                } else {
                    return Err("RemoveListElement : Esperado uma expressão".to_owned())
                };

                let list = match self.find_symbol(name.as_str()) {
                    Some(e) => e,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                if list.global {
                    instructions.push(Instruction::ReadGlobalVarFrom(list.address));
                } else {
                    instructions.push(Instruction::ReadVarFrom(list.address));
                }

                self.compile_expression(index, instructions)?;

                instructions.push(Instruction::RemoveFromListAtIndex);
            }
            CommandKind::IndexList => {
                let name = if let CommandArgument::Name(name) = cmd.arguments.remove(0) {
                    name
                } else {
                    return Err("IndexList : Esperado um nome".to_owned())
                };

                let index = if let CommandArgument::Expression(expr) = cmd.arguments.remove(0) {
                    expr
                } else {
                    return Err("IndexList : Esperado uma expressão".to_owned())
                };

                let dest_name = if let CommandArgument::Name(name) = cmd.arguments.remove(0) {
                    name
                } else {
                    return Err("IndexList : Esperado um nome".to_owned())
                };

                let dest = match self.find_or_add_symbol(dest_name.as_str(), true) {
                    Some(e) => e,
                    None => return Err(format!("Não foi possível encontrar ou declarar a variável {}", dest_name))
                };

                let list = match self.find_symbol(name.as_str()) {
                    Some(e) => e,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                if list.global {
                    instructions.push(Instruction::ReadGlobalVarFrom(list.address));
                } else {
                    instructions.push(Instruction::ReadVarFrom(list.address));
                }

                self.compile_expression(index, instructions)?;

                instructions.push(Instruction::IndexList);

                if dest.global {
                    instructions.push(Instruction::WriteGlobalVarTo(dest.address));
                } else {
                    instructions.push(Instruction::WriteVarTo(dest.address));
                }
            }
        }

        Ok(None)
    }

    pub fn begin_compiling_function(&mut self, address : usize, args : Vec<FunctionParameter>, name : String) -> Result<(), String> {
        let mut base_scope = ScopeInfo::new(SubScopeKind::Regular,
                                            self.next_var_address, false);

        self.next_var_address = 1;

        let mut args_kind = vec![];

        for arg in args {
            args_kind.push(arg.kind);

            base_scope.symbol_table.insert(arg.name, SymbolEntry::from(self.next_var_address, false, true));
            self.next_var_address += 1;
        }

        self.current_scope = ScopeKind::Function;
        self.functions.insert(name, FunctionInfo::from(address, args_kind));
        self.scopes.push(base_scope);

        Ok(())
    }

    pub fn compile_global_variable(&mut self, name : String, value : RawValue, writeable : bool, instructions : &mut Vec<Instruction>) -> Result<(), String> {
        if self.current_scope != ScopeKind::Global {
            return Err("Scope atual não é o global".to_owned());
        }

        let entry = match self.add_symbol(name, writeable) {
            Some(e) => e,
            None => return Err("Não foi possível adicionar o símbolo".to_owned())
        };

        instructions.push(Instruction::PushValMathB(value));
        instructions.push(Instruction::WriteGlobalVarTo(entry.address));

        Ok(())
    }

    pub fn compile_function_call(&self, id : usize, args : Vec<RawValue>, instructions : &mut Vec<Instruction>)
        -> Result<(), String>
    {
        let info = match self.get_function_info(id) {
            Some(i) => i,
            None => return Err(format!("Não encontrada função com id {}", id))
        };

        if info.arguments.len() != args.len() {
            return Err(format!("CompileFunctionCall : A função com ID {} espera {} argumentos, mas {} foram passados.", id,
                               info.arguments.len(), args.len()));
        }

        instructions.push(Instruction::MakeNewFrame(id));

        let mut index = 0usize;

        for arg in args {
            let expected = info.arguments[index];

            match &arg {
                &RawValue::Integer(_) => {
                    if expected != TypeKind::Integer && expected != TypeKind::Number {
                        return Err(format!("Tipo incompatível : Função espera {:?}, foi passado Inteiro", expected))
                    }
                }
                &RawValue::Number(_) => {
                    if expected != TypeKind::Number {
                        return Err(format!("Tipo incompatível : Função espera {:?}, foi passado Número", expected))
                    }
                }
                &RawValue::Text(_) => {
                    if expected != TypeKind::Text {
                        return Err(format!("Tipo incompatível : Função espera {:?}, foi passado Texto", expected))
                    }
                }
            }

            index += 1;

            instructions.push(Instruction::PushValMathB(arg));
            Instruction::WriteVarToLast(index + 1);
        }

        instructions.push(Instruction::SetLastFrameReady);

        Ok(())
    }

    pub fn end_compiling_function(&mut self) -> Result<(), String> {
        match self.scopes.pop() {
            Some(s) => {
                match s.scope_kind {
                    SubScopeKind::Regular => {}
                    _ => return Err("Fim da função encontrado, mas algum scope foi deixado aberto".to_owned()),
                }

                self.end_scope(s);

                self.current_scope = ScopeKind::Global;

                Ok(())
            }
            None => return Err("".to_owned())
        }
    }
}