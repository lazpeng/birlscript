use parser::{ Expression, ExpressionNode, Command, CommandArgument, MathOperator, MathValue, CommandKind };
use vm::Instruction;
use context::{ BIRL_GLOBAL_FUNCTION_ID, FunctionEntry };

#[derive(Debug, Clone)]
pub struct Variable {
    pub name : String,
    pub id : u64,
    pub writeable : bool,
}

pub enum CompilerHint {
    DeclareVar(Variable),
    ScopeStart,
    ScopeEnd,
}

pub struct Compiler {}

impl Compiler {
    fn get_inst_for_op(op : MathOperator) -> Option<Instruction> {
        match op {
            MathOperator::Plus => Some(Instruction::MainAdd),
            MathOperator::Minus => Some(Instruction::MainSub),
            MathOperator::Division => Some(Instruction::MainDiv),
            MathOperator::Multiplication => Some(Instruction::MainMul),
            _ => None,
        }
    }

    fn compile_sub_expression(expr : &Expression, offset : &mut usize, inst : &mut Vec<Instruction>,
                              func : &FunctionEntry, global : &Option<&FunctionEntry>) -> Result<(), String> {
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
                            match Compiler::compile_sub_expression(expr, offset, inst, func, &global) {
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
                    let mut on_global = false;

                    let id = match func.get_id_for(s.as_str()) {
                        Some(i) => {
                            if func.id == BIRL_GLOBAL_FUNCTION_ID {
                                on_global = true;
                            }

                            i
                        }
                        None => {
                            if func.id == BIRL_GLOBAL_FUNCTION_ID {
                                return Err(format!("Variável não encontrada : {}", s.as_str()));
                            }

                            if let &Some(ref g) = global {
                                on_global = true;

                                match g.get_id_for(s.as_str()) {
                                    Some(i) => i,
                                    None => return Err(format!("Variável não encontrada : {}", s.as_str())),
                                }
                            } else {
                                return Err("Erro interno : Função não é global e global é None".to_owned());
                            }
                        }
                    };

                    let inst = if on_global {
                        Instruction::ReadGlobalVarWithId(id)
                    } else {
                        Instruction::ReadVarWithId(id)
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

    pub fn compile_expression(expr : &Expression, inst : &mut Vec<Instruction>, func : &FunctionEntry,
                              global : &Option<&FunctionEntry>) -> Result<(), String> {
        let mut offset = 0usize;
        Compiler::compile_sub_expression(expr, &mut offset, inst, func, global)
    }

    fn get_id_and_globalness(name : &str, func : &FunctionEntry, global : &Option<&FunctionEntry>, is_global : &mut bool) -> Option<u64> {
        match func.get_id_for(name) {
            Some(id) => {
                if func.id == BIRL_GLOBAL_FUNCTION_ID {
                    *is_global = true;
                }

                Some(id)
            }
            None => {
                if func.id == BIRL_GLOBAL_FUNCTION_ID {
                    return None;
                }

                if let Some(g) = global {
                    match g.get_id_for(name) {
                        Some(id) => {
                            *is_global = true;
                            Some(id)
                        }
                        None => None,
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn compile_command(mut cmd : Command, func : &FunctionEntry, global : &Option<&FunctionEntry>,
        funcs : &Vec<FunctionEntry>, instructions : &mut Vec<Instruction>) -> Result<Option<CompilerHint>, String> {

        match cmd.kind {
            CommandKind::PrintDebug => {
                // Evaluate the single argument and print-debug it

                if cmd.arguments.len() != 1 {
                    return Err("Internal error : Debug print command has more than 1 argument (or less)".to_owned());
                }

                for arg in cmd.arguments {
                    match arg {
                        CommandArgument::Expression(expr) => {
                            match Compiler::compile_expression(&expr, instructions, &func, global) {
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
                            match Compiler::compile_expression(&expr, instructions, &func, global) {
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
                            match Compiler::compile_expression(&expr, instructions, &func, global) {
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

                let mut is_global = false;

                let id = match Compiler::get_id_and_globalness(name.as_str(), func, global, &mut is_global) {
                    Some(id) => id,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                let expr_arg = cmd.arguments.remove(0);

                match expr_arg {
                    CommandArgument::Expression(expr) => {
                        match Compiler::compile_expression(&expr, instructions, &func, global) {
                            Ok(_) => {}
                            Err(e) => return Err(e)
                        }
                    }
                    _ => return Err(format!("Erro interno : Esperado uma expressão depois do nome, encontrado {:?}", expr_arg)),
                }

                let inst = if is_global {
                    Instruction::WriteToGlobalVarWithId(id)
                } else {
                    Instruction::WriteToVarWithId(id)
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

                let is_global = func.id == BIRL_GLOBAL_FUNCTION_ID;

                let expr_arg = cmd.arguments.remove(0);

                match expr_arg {
                    CommandArgument::Expression(expr) => {
                        match Compiler::compile_expression(&expr, instructions, &func, global) {
                            Ok(_) => {}
                            Err(e) => return Err(e)
                        }
                    }
                    _ => return Err(format!("Erro interno : Esperado uma expressão depois do nome, encontrado {:?}", expr_arg)),
                }

                // Add the variable after the expression is parsed, so we can't use the variable before a value is set

                let id = func.next_var_id;

                let result = CompilerHint::DeclareVar(Variable { name, id, writeable : true });

                instructions.push(Instruction::CreateVarWithId(id));

                if is_global {
                    instructions.push(Instruction::WriteToGlobalVarWithId(id));
                } else {
                    instructions.push(Instruction::WriteToVarWithId(id));
                }

                return Ok(Some(result));
            }
            CommandKind::Return => {
                if cmd.arguments.is_empty() {
                    instructions.push(Instruction::PushNull);
                } else {
                    let expr_arg = cmd.arguments.remove(0);

                    match expr_arg {
                        CommandArgument::Expression(expr) => {
                            match Compiler::compile_expression(&expr, instructions, &func, global) {
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
                        match Compiler::compile_expression(&expr, instructions, &func, global) {
                            Ok(_) => {}
                            Err(e) => return Err(e)
                        }
                    }
                    _ => return Err(format!("Esperado uma expressão como argumento pro comando Return, encontrado {:?}", left_expr_arg)),
                }

                let right_expr_arg = cmd.arguments.remove(0);

                match right_expr_arg {
                    CommandArgument::Expression(expr) => {
                        match Compiler::compile_expression(&expr, instructions, &func, global) {
                            Ok(_) => {}
                            Err(e) => return Err(e)
                        }
                    }
                    _ => return Err(format!("Esperado uma expressão como argumento pro comando Return, encontrado {:?}", right_expr_arg)),
                }

                instructions.push(Instruction::CompareMainTop);
            }
            CommandKind::EndExecuteIf => {
                instructions.push(Instruction::EndExecuteIf);

                return Ok(Some(CompilerHint::ScopeEnd));
            },
            CommandKind::ExecuteIfEqual => {
                instructions.push(Instruction::ExecuteIfEqual);

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfNotEqual => {
                instructions.push(Instruction::ExecuteIfNotEqual);

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfEqualOrGreater => {
                instructions.push(Instruction::ExecuteIfGreaterOrEqual);

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfGreater => {
                instructions.push(Instruction::ExecuteIfGreater);

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfEqualOrLess => {
                instructions.push(Instruction::ExecuteIfLessOrEqual);

                return Ok(Some(CompilerHint::ScopeStart));
            },
            CommandKind::ExecuteIfLess => {
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

                for cf in funcs {
                    if cf.name == name {

                        instructions.push(Instruction::MakeNewFrame(cf.id));

                        // Check number of arguments

                        if cf.params.len() != cmd.arguments.len() {
                            return Err(format!("A função {} espera {} argumentos, mas {} foram passados",
                                name, cf.params.len(), cmd.arguments.len()));
                        }

                        // Push arguments and check their type

                        for index in 0..cf.params.len() {
                            let arg_arg = cmd.arguments.remove(0);

                            let expr = match arg_arg {
                                CommandArgument::Expression(e) => e,
                                _ => return Err("Erro interno : Era esperado um valor como argumento \
                                                    pro comando.".to_owned()),
                            };

                            let expected_type = cf.params[index].kind;
                            let arg_name = cf.params[index].name.as_str();

                            let mut arg_id = None;

                            for v in &cf.vars {
                                if v.name == arg_name {
                                    arg_id = Some(v.id);
                                }
                            }

                            if let None = arg_id {
                                return Err(format!("Erro interno : O parâmetro {} não está registrado como variável",
                                                   arg_name));
                            }

                            match Compiler::compile_expression(&expr, instructions, &func, global) {
                                Ok(_) => {}
                                Err(e) => return Err(e)
                            };

                            instructions.push(Instruction::AssertMainTopTypeCompatible(expected_type));

                            instructions.push(Instruction::WriteToLastFrameVarWithId(arg_id.unwrap()));
                        }

                        instructions.push(Instruction::SetLastFrameReady);

                        return Ok(None);
                    }
                }

                return Err(format!("A função {} não foi encontrada", name));
            }
            CommandKind::GetStringInput => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let mut is_global = false;

                let id = match Compiler::get_id_and_globalness(name.as_str(), func, global, &mut is_global) {
                    Some(id) => id,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                instructions.push(Instruction::ReadInput);

                if is_global {
                    instructions.push(Instruction::WriteToGlobalVarWithId(id));
                } else {
                    instructions.push(Instruction::WriteToVarWithId(id));
                }
            }
            CommandKind::GetIntegerInput => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let mut is_global = false;

                let id = match Compiler::get_id_and_globalness(name.as_str(), func, global, &mut is_global) {
                    Some(id) => id,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                instructions.push(Instruction::ReadInput);

                instructions.push(Instruction::ConvertToInt);

                if is_global {
                    instructions.push(Instruction::WriteToGlobalVarWithId(id));
                } else {
                    instructions.push(Instruction::WriteToVarWithId(id));
                }
            }
            CommandKind::GetNumberInput => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let mut is_global = false;

                let id = match Compiler::get_id_and_globalness(name.as_str(), func, global, &mut is_global) {
                    Some(id) => id,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                instructions.push(Instruction::ReadInput);

                instructions.push(Instruction::ConvertToNum);

                if is_global {
                    instructions.push(Instruction::WriteToGlobalVarWithId(id));
                } else {
                    instructions.push(Instruction::WriteToVarWithId(id));
                }
            }
            CommandKind::ConvertToInt => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let mut is_global = false;

                let id = match Compiler::get_id_and_globalness(name.as_str(), func, global, &mut is_global) {
                    Some(id) => id,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                if is_global {
                    instructions.push(Instruction::ReadGlobalVarWithId(id));
                } else {
                    instructions.push(Instruction::ReadVarWithId(id));
                }

                instructions.push(Instruction::ConvertToInt);

                if is_global {
                    instructions.push(Instruction::WriteToGlobalVarWithId(id));
                } else {
                    instructions.push(Instruction::WriteToVarWithId(id));
                }
            }
            CommandKind::ConvertToNum => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let mut is_global = false;

                let id = match Compiler::get_id_and_globalness(name.as_str(), func, global, &mut is_global) {
                    Some(id) => id,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                if is_global {
                    instructions.push(Instruction::ReadGlobalVarWithId(id));
                } else {
                    instructions.push(Instruction::ReadVarWithId(id));
                }

                instructions.push(Instruction::ConvertToNum);

                if is_global {
                    instructions.push(Instruction::WriteToGlobalVarWithId(id));
                } else {
                    instructions.push(Instruction::WriteToVarWithId(id));
                }
            }
            CommandKind::IntoString => {
                let name_arg = cmd.arguments.remove(0);

                let name = match name_arg {
                    CommandArgument::Name(s) => s,
                    _ => return Err("Erro interno : Esperado um nome pra GetInput*".to_owned()),
                };

                let mut is_global = false;

                let id = match Compiler::get_id_and_globalness(name.as_str(), func, global, &mut is_global) {
                    Some(id) => id,
                    None => return Err(format!("Variável {} não encontrada", name))
                };

                if is_global {
                    instructions.push(Instruction::ReadGlobalVarWithId(id));
                } else {
                    instructions.push(Instruction::ReadVarWithId(id));
                }

                instructions.push(Instruction::ConvertToString);

                if is_global {
                    instructions.push(Instruction::WriteToGlobalVarWithId(id));
                } else {
                    instructions.push(Instruction::WriteToVarWithId(id));
                }
            }
        }

        Ok(None)
    }
}