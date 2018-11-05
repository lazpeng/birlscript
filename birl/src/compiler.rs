use std::collections::HashMap;
use parser::{ Expression, ExpressionNode, FunctionParameter, Command, TypeKind, CommandArgument, MathOperator, MathValue, CommandKind };
use vm::Instruction;
use context::RawValue;

#[derive(Debug)]
enum SubScopeKind {
    ForLoop,
    WhileLoop,
    ExecuteIf,
    Regular,
}

#[derive(PartialEq)]
enum ScopeKind {
    Function,
    Global
}

#[derive(Debug)]
struct SymbolEntry {
    address : usize,
    global : bool,
}

impl SymbolEntry {
    fn from(address : usize, global : bool) -> SymbolEntry {
        SymbolEntry { address, global }
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
        symbol_table.insert("TREZE".to_owned(), SymbolEntry::from(0, is_global));

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
            MathOperator::Plus => Some(Instruction::MainAdd),
            MathOperator::Minus => Some(Instruction::MainSub),
            MathOperator::Division => Some(Instruction::MainDiv),
            MathOperator::Multiplication => Some(Instruction::MainMul),
            _ => None,
        }
    }

    fn compile_sub_expression(&self, expr : &Expression, offset : &mut usize, inst : &mut Vec<Instruction>)
            -> Result<(), String> {
        let mut buffer : Vec<Instruction> = vec![];

        let mut last_imp_op : Option<MathOperator> = None;

        let mut op_stack : Vec<MathOperator> = vec![];

        loop {
            if *offset >= expr.nodes.len() {
                break;
            }

            let ref current = expr.nodes[*offset];
            *offset += 1;

            match current {
                &ExpressionNode::Operator(op) => {
                    match op {
                        MathOperator::ParenthesisLeft => {
                            match self.compile_sub_expression(expr, offset, inst) {
                                Ok(_) => {}
                                Err(e) => return Err(e),
                            }

                            if let Some(op) = last_imp_op {
                                let i = match Compiler::get_inst_for_op(op) {
                                    Some(i) => i,
                                    None => return Err("Invalid operator in important operator".to_owned()),
                                };

                                buffer.push(i);

                                last_imp_op = None;
                            }
                        }
                        MathOperator::ParenthesisRight => break,
                        MathOperator::Plus | MathOperator::Minus => {
                            op_stack.push(op);
                        }
                        _ => {
                            if let Some(_) = last_imp_op {
                                return Err("Two subsequent important operators".to_owned());
                            }

                            last_imp_op = Some(op);
                        }
                    }
                }
                &ExpressionNode::Value(ref v) => {
                    match v {
                        &MathValue::Integer(i) => {
                            buffer.push(Instruction::PushMainInt(i))
                        }
                        &MathValue::Number(n) => {
                            buffer.push(Instruction::PushMainNum(n))
                        }
                        &MathValue::Text(ref s) => {
                            buffer.push(Instruction::PushMainStr(s.clone()))
                        }
                    }

                    if let Some(op) = last_imp_op {
                        let i = match Compiler::get_inst_for_op(op) {
                            Some(i) => i,
                            None => return Err("Invalid operator in important operator".to_owned()),
                        };

                        buffer.push(i);

                        last_imp_op = None;
                    }
                }
                &ExpressionNode::Symbol(ref s) => {
                    let entry = match self.find_symbol(s) {
                        Some(e) => e,
                        None => return Err(format!("Variável {} não encontrada", s))
                    };

                    let inst = if entry.global {
                        Instruction::ReadGlobalVarFromAddress(entry.address)
                    } else {
                        Instruction::ReadVarFromAddress(entry.address)
                    };

                    buffer.push(inst);

                    if let Some(op) = last_imp_op {
                        let i = match Compiler::get_inst_for_op(op) {
                            Some(i) => i,
                            None => return Err("Invalid operator in important operator".to_owned()),
                        };

                        buffer.push(i);

                        last_imp_op = None;
                    }
                }
            }
        }

        for op in op_stack {
            let inst = match Compiler::get_inst_for_op(op) {
                Some(i) => i,
                None => return Err("Erro: Invalid operator on stack".to_owned()),
            };

            buffer.push(inst);
        }

        for i in buffer {
            inst.push(i);
        }

        Ok(())
    }

    pub fn compile_expression(&self, expr : &Expression, inst : &mut Vec<Instruction>) -> Result<(), String> {
        let mut offset = 0usize;
        self.compile_sub_expression(expr, &mut offset, inst)
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

    fn get_function_info(&self, id : usize) -> Option<&FunctionInfo> {
        for (_, f) in &self.functions {
            if f.address == id {
                return Some(f);
            }
        }

        None
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
                            match self.compile_expression(&expr, instructions) {
                                Ok(_) => {},
                                Err(e) => return Err(e),
                            };

                            instructions.push(Instruction::MainPrintDebug);
                        }
                        _ => return Err("Erro : Um argumento diferente de valor foi passado pra print. Erro interno.".to_owned()),
                    }
                }
            }
            CommandKind::Print => {
                for arg in cmd.arguments {
                    match arg {
                        CommandArgument::Expression(expr) => {
                            match self.compile_expression(&expr, instructions) {
                                Ok(_) => {},
                                Err(e) => return Err(e),
                            };

                            instructions.push(Instruction::MainPrint);
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
                            match self.compile_expression(&expr, instructions) {
                                Ok(_) => {},
                                Err(e) => return Err(e),
                            };

                            instructions.push(Instruction::MainPrint);
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

                let expr_arg = cmd.arguments.remove(0);

                match expr_arg {
                    CommandArgument::Expression(expr) => {
                        match self.compile_expression(&expr, instructions) {
                            Ok(_) => {}
                            Err(e) => return Err(e)
                        }
                    }
                    _ => return Err(format!("Erro interno : Esperado uma expressão depois do nome, encontrado {:?}", expr_arg)),
                }

                let inst = if entry.global {
                    Instruction::WriteToGlobalVarAtAddress(entry.address)
                } else {
                    Instruction::WriteToVarAtAddress(entry.address)
                };

                instructions.push(inst);
            }
            CommandKind::Declare => {
                if cmd.arguments.len() != 2 {
                    return Err(format!("O comando BORA espera 2 argumentos, mas {} foram passados (Erro interno)", cmd.arguments.len()));
                }

                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(n) => n,
                    _ => return Err(format!("Erro interno : Esperado um nome pro BORA, encontrado {:?}", name_arg)),
                };

                let is_global = self.current_scope == ScopeKind::Global;

                let expr_arg = cmd.arguments.remove(0);

                match expr_arg {
                    CommandArgument::Expression(expr) => {
                        match self.compile_expression(&expr, instructions) {
                            Ok(_) => {}
                            Err(e) => return Err(e)
                        }
                    }
                    _ => return Err(format!("Erro interno : Esperado uma expressão depois do nome, encontrado {:?}", expr_arg)),
                }

                // Add the variable after the expression is parsed, so we can't use the variable before a value is set

                let address = self.next_var_address;
                self.next_var_address += 1;

                match self.scopes.last_mut() {
                    Some(s) => s.symbol_table.insert(name, SymbolEntry::from(address, is_global)),
                    None => return Err(format!("Scopes é vazio"))
                };

                if is_global {
                    instructions.push(Instruction::WriteToGlobalVarAtAddress(address));
                } else {
                    instructions.push(Instruction::WriteToVarAtAddress(address));
                }
            }
            CommandKind::Return => {
                if cmd.arguments.is_empty() {
                    instructions.push(Instruction::PushNull);
                } else {
                    let expr_arg = cmd.arguments.remove(0);

                    match expr_arg {
                        CommandArgument::Expression(expr) => {
                            match self.compile_expression(&expr, instructions) {
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
                        match self.compile_expression(&expr, instructions) {
                            Ok(_) => {}
                            Err(e) => return Err(e)
                        }
                    }
                    _ => return Err(format!("Esperado uma expressão como argumento pro comando Return, encontrado {:?}", left_expr_arg)),
                }

                let right_expr_arg = cmd.arguments.remove(0);

                match right_expr_arg {
                    CommandArgument::Expression(expr) => {
                        match self.compile_expression(&expr, instructions) {
                            Ok(_) => {}
                            Err(e) => return Err(e)
                        }
                    }
                    _ => return Err(format!("Esperado uma expressão como argumento pro comando Return, encontrado {:?}", right_expr_arg)),
                }

                instructions.push(Instruction::CompareMainTop);
            }
            CommandKind::EndSubScope => {
                let scope_info = match self.scopes.pop() {
                    Some(s) => s,
                    None => return Err(format!("FIM fora de qualquer scope"))
                };

                match scope_info.scope_kind {
                    SubScopeKind::ExecuteIf => instructions.push(Instruction::EndExecuteIf),
                    _ => {}
                }

                self.end_scope(scope_info);

                return Ok(Some(CompilerHint::ScopeEnd));
            },
            CommandKind::ExecuteIfEqual => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::ExecuteIf,
                                                self.next_var_address, is_global));
                instructions.push(Instruction::ExecuteIfEqual);

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfNotEqual => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::ExecuteIf,
                                                self.next_var_address, is_global));
                instructions.push(Instruction::ExecuteIfNotEqual);

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfEqualOrGreater => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::ExecuteIf,
                                                self.next_var_address, is_global));
                instructions.push(Instruction::ExecuteIfGreaterOrEqual);

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfGreater => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::ExecuteIf,
                                                self.next_var_address, is_global));
                instructions.push(Instruction::ExecuteIfGreater);

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfEqualOrLess => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::ExecuteIf,
                                                self.next_var_address, is_global));
                instructions.push(Instruction::ExecuteIfLessOrEqual);

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfLess => {
                let is_global = self.current_scope == ScopeKind::Global;
                self.scopes.push(ScopeInfo::new(SubScopeKind::ExecuteIf,
                                                self.next_var_address, is_global));
                instructions.push(Instruction::ExecuteIfLess);

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::Call => {
                // First argument is the function name

                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(n) => n,
                    _ => return Err(format!("Erro interno : Esperado um nome pra função")),
                };

                let info = match self.functions.get(name.as_str()) {
                    Some(i) => i,
                    None => return Err(format!("Não existe função com nome {}", name))
                };

                instructions.push(Instruction::MakeNewFrame(info.address));

                if info.arguments.len() != cmd.arguments.len() {
                    return Err(format!("A função {} espera {} argumentos, mas {} foram passados",
                                       name, info.arguments.len(), cmd.arguments.len()));
                }

                for index in 0..info.arguments.len() {
                    let arg_arg = cmd.arguments.remove(0);

                    let expr = match arg_arg {
                        CommandArgument::Expression(e) => e,
                        _ => return Err("Erro interno : Era esperado um valor como argumento \
                                                    pro comando.".to_owned()),
                    };

                    let expected_type = info.arguments[index];

                    // The parameter address is, in this case, index + 1 (because the address 0 is reserved to
                    // the return value)

                    match self.compile_expression(&expr, instructions) {
                        Ok(_) => {}
                        Err(e) => return Err(e)
                    };

                    instructions.push(Instruction::AssertMainTopTypeCompatible(expected_type));

                    instructions.push(Instruction::WriteToLastFrameVarAtAddress(index + 1));
                }

                instructions.push(Instruction::SetLastFrameReady);
            }
            CommandKind::GetStringInput => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let entry = match self.find_symbol(name.as_str()) {
                    Some(e) => e,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                instructions.push(Instruction::ReadInput);

                if entry.global {
                    instructions.push(Instruction::WriteToGlobalVarAtAddress(entry.address));
                } else {
                    instructions.push(Instruction::WriteToVarAtAddress(entry.address));
                }
            }
            CommandKind::GetIntegerInput => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let entry = match self.find_symbol(name.as_str()) {
                    Some(e) => e,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                instructions.push(Instruction::ReadInput);

                instructions.push(Instruction::ConvertToInt);

                if entry.global {
                    instructions.push(Instruction::WriteToGlobalVarAtAddress(entry.address));
                } else {
                    instructions.push(Instruction::WriteToVarAtAddress(entry.address));
                }
            }
            CommandKind::GetNumberInput => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let entry = match self.find_symbol(name.as_str()) {
                    Some(e) => e,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                instructions.push(Instruction::ReadInput);

                instructions.push(Instruction::ConvertToNum);

                if entry.global {
                    instructions.push(Instruction::WriteToGlobalVarAtAddress(entry.address));
                } else {
                    instructions.push(Instruction::WriteToVarAtAddress(entry.address));
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
                    instructions.push(Instruction::ReadGlobalVarFromAddress(entry.address));
                } else {
                    instructions.push(Instruction::ReadVarFromAddress(entry.address));
                }

                instructions.push(Instruction::ConvertToInt);

                if entry.global {
                    instructions.push(Instruction::WriteToGlobalVarAtAddress(entry.address));
                } else {
                    instructions.push(Instruction::WriteToVarAtAddress(entry.address));
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
                    instructions.push(Instruction::ReadGlobalVarFromAddress(entry.address));
                } else {
                    instructions.push(Instruction::ReadVarFromAddress(entry.address));
                }

                instructions.push(Instruction::ConvertToInt);

                if entry.global {
                    instructions.push(Instruction::WriteToGlobalVarAtAddress(entry.address));
                } else {
                    instructions.push(Instruction::WriteToVarAtAddress(entry.address));
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
                    instructions.push(Instruction::ReadGlobalVarFromAddress(entry.address));
                } else {
                    instructions.push(Instruction::ReadVarFromAddress(entry.address));
                }

                instructions.push(Instruction::ConvertToInt);

                if entry.global {
                    instructions.push(Instruction::WriteToGlobalVarAtAddress(entry.address));
                } else {
                    instructions.push(Instruction::WriteToVarAtAddress(entry.address));
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

            base_scope.symbol_table.insert(arg.name, SymbolEntry::from(self.next_var_address, false));
            self.next_var_address += 1;
        }

        self.current_scope = ScopeKind::Function;
        self.functions.insert(name, FunctionInfo::from(address, args_kind));
        self.scopes.push(base_scope);

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
            index += 1;

            match arg {
                RawValue::Text(t) => {
                    if expected == TypeKind::Text {
                        instructions.push(Instruction::PushMainStr(t));
                    } else {
                        return Err(format!("A função esperava um {:?} como argumento, mas um Texto foi passado", expected));
                    }
                }
                RawValue::Number(n) => {
                    if expected == TypeKind::Number {
                        instructions.push(Instruction::PushMainNum(n));
                    } else {
                        return Err(format!("A função esperava um {:?} como argumento, mas um Num foi passado", expected));
                    }
                }
                RawValue::Integer(i) => {
                    match expected {
                        TypeKind::Text => return Err("A função esperava um texto, mas um Int foi passado".to_owned()),
                        TypeKind::Integer => instructions.push(Instruction::PushMainInt(i)),
                        TypeKind::Number => instructions.push(Instruction::PushMainNum(i as f64)),
                    }
                }
            }

            instructions.push(Instruction::WriteToLastFrameVarAtAddress(index + 1));
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