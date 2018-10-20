use birl::parser::{ Expression, ExpressionNode, Command, CommandArgument, MathOperator, MathValue, CommandKind };
use birl::vm::Instruction;
use birl::context::{ BIRL_GLOBAL_FUNCTION_ID, FunctionEntry };

pub struct Variable {
    pub name : String,
    pub id : u64,
    pub writeable : bool,
}

pub enum CompilerHint {
    DeclareVar(Variable),
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

                let id = {
                    match func.get_id_for(name.as_str()) {
                        Some(id) => {
                            if func.id == BIRL_GLOBAL_FUNCTION_ID {
                                is_global = true;
                            }

                            id
                        }
                        None => {
                            if func.id == BIRL_GLOBAL_FUNCTION_ID {
                                return Err(format!("Variável {} não encontrada", name));
                            }

                            if let Some(g) = global {
                                match g.get_id_for(name.as_str()) {
                                    Some(id) => {
                                        is_global = true;
                                        id
                                    }
                                    None => return Err(format!("Variável {} não encontrada", name)),
                                }
                            } else {
                                return Err(format!("Erro interno : Func não é global, mas global é None"));
                            }
                        }
                    }
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
            CommandKind::EndExecuteIf => instructions.push(Instruction::EndExecuteIf),
            CommandKind::ExecuteIfEqual => instructions.push(Instruction::ExecuteIfEqual),
            CommandKind::ExecuteIfNotEqual => instructions.push(Instruction::ExecuteIfNotEqual),
            CommandKind::ExecuteIfEqualOrGreater => instructions.push(Instruction::ExecuteIfGreaterOrEqual),
            CommandKind::ExecuteIfGreater => instructions.push(Instruction::ExecuteIfGreater),
            CommandKind::ExecuteIfEqualOrLess => instructions.push(Instruction::ExecuteIfLessOrEqual),
            CommandKind::ExecuteIfLess => instructions.push(Instruction::ExecuteIfLess),
            CommandKind::Call => {
                // TODO
            }
        }

        Ok(None)
    }
}