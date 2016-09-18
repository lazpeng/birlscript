
use vm;
use parser;

/// Implementação dos comandos
mod cmd {
    use vm::signal::Signal;
    use value;
    use vm;

    pub fn print(args: Vec<String>, vm: &mut vm::VM) -> Option<Signal> {
        for arg in &args {
            print!("{}", value::parse_expr(arg, vm));
        }
        None
    }

    pub fn println(args: Vec<String>, vm: &mut vm::VM) -> Option<Signal> {
        for arg in &args {
            print!("{}", value::parse_expr(arg, vm));
        }
        println!("");
        None
    }
}

pub fn run(cmd: parser::Command, vm: &mut vm::VM) -> Option<vm::signal::Signal> {
    use parser::Command::*;
    match cmd {
        Print(a) => cmd::print(a, vm),
        Println(a) => cmd::println(a, vm),
        _ => None,
    }
}
