//! Hosts the runtime for the birlscript language

// BIG TODO : Add default variables

use vm::{ VirtualMachine, ExecutionStatus };
use parser::{ parse_line, TypeKind, ParserResult, IntegerType, FunctionDeclaration };
use compiler::{ Compiler, CompilerHint };

use std::io::{ BufRead, BufReader, Write };
use std::fs::File;

pub const BIRL_COPYRIGHT : &'static str 
    = "© 2016 - 2018 Rafael Rodrigues Nakano <lazpeng@gmail.com>";
pub const BIRL_VERSION : &'static str 
    = "BirlScript v2.0.0-alpha";
pub const BIRL_MAIN_FUNCTION : &str 
    = "SHOW";

pub const BIRL_MAIN_FUNCTION_ID     : usize = 1;
pub const BIRL_GLOBAL_FUNCTION_ID   : usize = 0;
pub const BIRL_RET_VAL_VAR_ADDRESS  : usize = 0;

#[derive(Debug, Clone, PartialEq)]
pub enum RawValue {
    Text(String),
    Integer(IntegerType),
    Number(f64)
}

impl RawValue {
    pub fn get_kind(&self) -> TypeKind {
        match &self {
            &RawValue::Integer(_) => TypeKind::Integer,
            &RawValue::Number(_) => TypeKind::Number,
            &RawValue::Text(_) => TypeKind::Text,
        }
    }
}

pub struct Context {
    pub vm : VirtualMachine,
    has_main : bool,
    compiler : Compiler,
    current_code_id : usize,
}

impl Context {
    /// Alias for vm.set_stdout().
    pub fn set_stdout(&mut self, write: Option<Box<Write>>) -> Option<Box<Write>>{
        self.vm.set_stdout(write)
    }

    /// Alias for vm.set_stdin().
    pub fn set_stdin(&mut self, read: Option<Box<BufRead>>) -> Option<Box<BufRead>>{
        self.vm.set_stdin(read)
    }

    pub fn new() -> Context {
        let mut vm = VirtualMachine::new();
        let _ = vm.add_new_code(); // For global
        let _ = vm.add_new_code(); // For main

        Context {
            vm,
            has_main : false,
            compiler : Compiler::new(),
            current_code_id : 0,
        }
    }

    fn add_function(&mut self, f : FunctionDeclaration) -> Result<(), String> {
        let is_main = f.name == BIRL_MAIN_FUNCTION;
        if is_main {
            if self.has_main {
                return Err("Erro: Múltipla declaração da função principal".to_owned());
            }
            if f.arguments.len() != 0 {
                return Err("Erro : Declaração da função principal inválida : A função principal não deve pedir argumentos".to_owned());
            }
            self.has_main = true;
        }

        let id = if is_main {
            BIRL_MAIN_FUNCTION_ID
        } else {
            self.vm.add_new_code()
        };

        self.compiler.begin_compiling_function(id, f.arguments, f.name)?;

        self.current_code_id = id;

        Ok(())
    }

    pub fn set_interactive_mode(&mut self) {
        self.vm.set_interactive_mode();
    }
    
    pub fn end_function(&mut self) -> Result<(), String>{
        self.compiler.end_compiling_function()?;

        self.current_code_id = BIRL_GLOBAL_FUNCTION_ID;

        Ok(())
    }

    pub fn process_line(&mut self, line : &str) -> Result<Option<CompilerHint>, String> {

        let result = match parse_line(line) {
            Ok(r) => r,
            Err(e) => return Err(e)
        };

        match result {
            ParserResult::Command(cmd) => {
                let hint = {
                    let instructions = match self.vm.get_code_for(self.current_code_id) {
                        Some(i) => i,
                        None => return Err(format!("Erro ao pegar o código para a função atual"))
                    };

                    match self.compiler.compile_command(cmd, instructions) {
                        Ok(hint) => hint,
                        Err(e) => return Err(e)
                    }
                };

                return Ok(hint);
            }
            ParserResult::FunctionEnd => self.end_function()?,
            ParserResult::FunctionStart(func) => self.add_function(func)?,
            ParserResult::Nothing => return Ok(None)
        }

        Ok(None)
    }

    pub fn add_source_string(&mut self, string : String) -> Result<(), String> {
        let reader = BufReader::new(string.as_bytes());

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    match self.process_line(line.as_str()) {
                        Ok(_) => {}
                        Err(e) => return Err(e)
                    }
                }
                Err(e) => return Err(format!("{:?}", e))
            }
        }

        Ok(())
    }

    pub fn add_file(&mut self, filename : &str) -> Result<(), String> {
        let file = match File::open(filename) {
            Ok(f) => f,
            Err(e) => return Err(format!("{:?}", e)),
        };

        let mut line_num = 0usize;

        let reader = BufReader::new(file);

        for line in reader.lines() {
            line_num += 1;
            match line {
                Ok(line) => {
                    match self.process_line(line.as_str()) {
                        Ok(_) => {}
                        Err(e) => return Err(format!("(Linha {}) : {:?}", line_num, e))
                    }
                }
                Err(e) => return Err(format!("(Linha {}) : {:?}", line_num, e))
            }
        }

        Ok(())
    }

    pub fn call_function_by_id(&mut self, id : usize, args : Vec<RawValue>) -> Result<(), String> {
        let mut instructions = vec![];

        match self.compiler.compile_function_call(id, args, &mut instructions) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        for i in instructions {
            match self.vm.run(i) {
                Ok(_) => {}
                Err(e) => return Err(e)
            }
        }

        Ok(())
    }

    pub fn execute_next_instruction(&mut self) -> Result<ExecutionStatus, String> {
        self.vm.execute_next_instruction()
    }

    pub fn start_program(&mut self) -> Result<(), String> {
        match self.call_function_by_id(BIRL_GLOBAL_FUNCTION_ID, vec![]) {
            Ok(_) => {}
            Err(e) => return Err(e)
        }

        loop {
            match self.execute_next_instruction() {
                Ok(ExecutionStatus::Normal) => {}
                Ok(ExecutionStatus::Returned) => {}
                Ok(ExecutionStatus::Halt) => break,
                Ok(ExecutionStatus::Quit) => break,
                Err(e) => return Err(e)
            }
        }

        self.vm.unset_quit();

        if self.has_main {
            match self.call_function_by_id(BIRL_MAIN_FUNCTION_ID, vec![]) {
                Ok(_) => {}
                Err(e) => return Err(e)
            }

            loop {
                match self.execute_next_instruction() {
                    Ok(ExecutionStatus::Normal) => {}
                    Ok(ExecutionStatus::Returned) => {}
                    Ok(ExecutionStatus::Halt) => break,
                    Ok(ExecutionStatus::Quit) => return Ok(()),
                    Err(e) => return Err(e)
                }
            }
        }

        Ok(())
    } 
    
    pub fn print_version() {
        println!("{}", BIRL_VERSION);
        println!("{}", BIRL_COPYRIGHT);
    }
}
