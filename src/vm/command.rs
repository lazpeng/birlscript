
use vm;
use parser;

/// Implementação dos comandos
mod cmd {
    use vm::signal::Signal;
    use eval::{self, Value};
    use vm::variable::Variable;
    use vm::VM;
    use parser::{self, Command};
    use vm::comparision::Comparision;
    use vm::parameter::Parameter;

    pub fn cmd_move(name: String, value: String, vm: &mut VM) -> Option<Signal> {
        let value = eval::evaluate(&value, vm);
        vm.modify_variable(&name, value);
        None
    }

    pub fn clear(var: String, vm: &mut VM) -> Option<Signal> {
        let val = Value::NullOrEmpty;
        vm.modify_variable(&var, val);
        None
    }

    pub fn decl(var: String, vm: &mut VM) -> Option<Signal> {
        let result = Variable::from(&var, Value::NullOrEmpty);
        vm.declare_variable(result);
        None
    }

    pub fn declwv(var: String, val: String, vm: &mut VM) -> Option<Signal> {
        let val = eval::evaluate(&val, vm);
        let result = Variable::from(&var, val);
        vm.declare_variable(result);
        None
    }

    pub fn jump(sect: String, vm: &mut VM) -> Option<Signal> {
        let (arg_list, sect_name) = parser::parse_section_call_params(&sect);
        let section = vm.get_section(&sect_name).clone();
        let expected = section.args;
        let mut index = 0;
        let mut param_list: Vec<Parameter> = vec![];
        for arg in &arg_list {
            let val = eval::evaluate(arg, vm);
            let v = Variable::from(&expected[index].id, val);
            let param = Parameter { var: v };
            param_list.push(param);
            index += 1;
        }
        vm.start_section(&sect_name, param_list);
        None
    }

    pub fn cmp(left: String, right: String, vm: &mut VM) -> Option<Signal> {
        let (left, right) = (eval::evaluate(&left, vm), eval::evaluate(&right, vm));
        vm.compare(left, right);
        None
    }

    pub fn cmp_eq(command: Command, vm: &mut VM) -> Option<Signal> {
        if vm.last_cmp_is(Comparision::Equal) {
            super::run(command, vm)
        } else {
            None
        }
    }

    pub fn cmp_neq(command: Command, vm: &mut VM) -> Option<Signal> {
        if vm.last_cmp_is(Comparision::NEqual) {
            super::run(command, vm)
        } else {
            None
        }
    }

    pub fn cmp_less(command: Command, vm: &mut VM) -> Option<Signal> {
        if vm.last_cmp_is(Comparision::Less) {
            super::run(command, vm)
        } else {
            None
        }
    }

    pub fn cmp_lesseq(command: Command, vm: &mut VM) -> Option<Signal> {
        if vm.last_cmp_is(Comparision::Less) || vm.last_cmp_is(Comparision::Equal) {
            super::run(command, vm)
        } else {
            None
        }
    }

    pub fn cmp_more(command: Command, vm: &mut VM) -> Option<Signal> {
        if vm.last_cmp_is(Comparision::More) {
            super::run(command, vm)
        } else {
            None
        }
    }

    pub fn cmp_moreeq(command: Command, vm: &mut VM) -> Option<Signal> {
        if vm.last_cmp_is(Comparision::More) || vm.last_cmp_is(Comparision::Equal) {
            super::run(command, vm)
        } else {
            None
        }
    }

    pub fn print(args: Vec<String>, vm: &mut VM) -> Option<Signal> {
        for arg in &args {
            print!("{}", eval::evaluate(arg, vm));
        }
        None
    }

    pub fn println(args: Vec<String>, vm: &mut VM) -> Option<Signal> {
        for arg in &args {
            print!("{}", eval::evaluate(arg, vm));
        }
        println!("");
        None
    }

    pub fn quit(code: String, vm: &mut VM) -> Option<Signal> {
        let result = if code != "" {
            eval::evaluate(&code, vm)
        } else {
            Value::Num(0.0)
        };
        if let Value::Num(c) = result {
            Some(Signal::Quit(c as i32))
        } else {
            panic!("Erro ao tentar quitar com código invalido (não-número)")
        }
    }

    pub fn cmd_return(val: Option<String>, vm: &mut VM) -> Option<Signal> {
        let val = match val {
            Some(a) => Some(eval::evaluate(&a, vm)),
            None => None,
        };
        vm.section_return(val);
        Some(Signal::Return)
    }

    pub fn input(var: String, vm: &mut VM) -> Option<Signal> {
        let inp = get_input();
        vm.modify_variable(&var, Value::Str(inp.to_string()));
        None
    }

    pub fn input_upper(var: String, vm: &mut VM) -> Option<Signal> {
        let inp = get_input().to_uppercase();
        vm.modify_variable(&var, Value::Str(inp.to_string()));
        None
    }

    fn get_input() -> String {
        use std::io;
        let sin = io::stdin();
        let mut res = String::new();
        sin.read_line(&mut res).expect("Erro lendo da entrada padrão");
        res.trim().to_string()
    }
}

pub fn run(cmd: parser::Command, vm: &mut vm::VM) -> Option<vm::signal::Signal> {
    use parser::Command::*;
    match cmd {
        Move(a, b) => cmd::cmd_move(a, b, vm),
        Clear(a) => cmd::clear(a, vm),
        Decl(a) => cmd::decl(a, vm),
        DeclWV(a, b) => cmd::declwv(a, b, vm),
        Jump(a) => cmd::jump(a, vm),
        Cmp(a, b) => cmd::cmp(a, b, vm),
        CmpEq(a) => cmd::cmp_eq(*a, vm),
        CmpNEq(a) => cmd::cmp_neq(*a, vm),
        CmpLess(a) => cmd::cmp_less(*a, vm),
        CmpLessEq(a) => cmd::cmp_lesseq(*a, vm),
        CmpMore(a) => cmd::cmp_more(*a, vm),
        CmpMoreEq(a) => cmd::cmp_moreeq(*a, vm),
        Print(a) => cmd::print(a, vm),
        Println(a) => cmd::println(a, vm),
        Quit(a) => cmd::quit(a, vm),
        Return(a) => cmd::cmd_return(a, vm),
        Input(a) => cmd::input(a, vm),
        InputUpper(a) => cmd::input_upper(a, vm),
    }
}
