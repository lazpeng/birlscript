//! Hosts the runtime for the birlscript language

use birl::vm::{ Instruction, VirtualMachine };
use birl::parser::{ parse_line, FunctionParameter, ParserResult, FunctionDeclaration };
use birl::compiler::{ Compiler, Variable };

use std::io::{ Write, stdin, stdout };

pub const BIRL_COPYRIGHT : &'static str = "© 2016, 2017, 2018 Rafael Rodrigues Nakano <lazpeng@gmail.com";
pub const BIRL_VERSION : &'static str = "BirlScript v2.0.0-alpha";

pub const BIRL_MAIN_FUNCTION : &str = "SHOW";

pub const BIRL_MAIN_FUNCTION_ID : u64 = 1;
pub const BIRL_GLOBAL_FUNCTION_ID : u64 = 0;

pub const BIRL_RET_VAL_VAR_ID : u64 = 0;

#[derive(Clone)]
pub struct FunctionEntry {
    pub name : String,
    pub id : u64,
    pub body : Vec<Instruction>,
    pub params : Vec<FunctionParameter>,
    pub vars : Vec<Variable>,
    pub next_var_id : u64,
}

impl FunctionEntry {
    pub fn get_id_for(&self, var : &str) -> Option<u64> {
        for v in &self.vars {
            if v.name == var {
                return Some(v.id);
            }
        }

        None
    }

    pub fn from(name : String, id : u64, params : Vec<FunctionParameter>) -> FunctionEntry {
        FunctionEntry {
            name,
            id,
            body : vec![],
            params,
            vars : vec![Variable { name : "TREZE".to_owned(), id : BIRL_RET_VAL_VAR_ID, writeable : true }],
            next_var_id : 1,
        }
    }

    pub fn add_var(&mut self, name : String, writeable : bool) -> Result<u64, String> {
        for v in &self.vars {
            if name == v.name.as_str() {
                return Err(format!("A variável {} já está declarada.", name.as_str()));
            }
        }

        let id = self.next_var_id;

        self.vars.push(Variable { name, id, writeable });

        self.next_var_id += 1;

        Ok(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Scope {
    Global,
    Function,
}

pub struct Context {
    vm : VirtualMachine,
    functions : Vec<FunctionEntry>,
    scope : Scope,
    has_main : bool,
    next_function_id : u64,
}

impl Context {
    fn new_global() -> FunctionEntry {
        FunctionEntry::from("__global__".to_owned(), BIRL_GLOBAL_FUNCTION_ID, vec![])
    }

    pub fn get_functions(&self) -> &[FunctionEntry] {
        &self.functions
    }

    pub fn new() -> Context {
        Context {
            vm : VirtualMachine::new(),
            functions : vec![Context::new_global()],
            scope : Scope::Global,
            has_main : false,
            next_function_id : 2,
        }
    }

    fn get_entry_by_id(&self, id : u64) -> Option<&FunctionEntry> {
        for e in &self.functions {
            if e.id == id {
                return Some(e);
            }
        }

        None
    }

    fn get_entry_by_id_mut(&mut self, id : u64) -> Option<&mut FunctionEntry> {
        for e in &mut self.functions {
            if e.id == id {
                return Some(e);
            }
        }

        None
    }

    fn get_global_mut(&mut self) -> Option<&mut FunctionEntry> {
        self.get_entry_by_id_mut(BIRL_GLOBAL_FUNCTION_ID)
    }

    fn get_global(&self) -> Option<&FunctionEntry> {
        self.get_entry_by_id(BIRL_GLOBAL_FUNCTION_ID)
    }

    fn add_function(&mut self, f : FunctionDeclaration) -> Result<(), String> {
        if self.scope != Scope::Global {
            return Err("Erro: Declarando uma função fora do scope global".to_owned());
        }

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

        let mut vars : Vec<Variable> = vec![];

        let mut next_var_id = 0u64;

        // Register all parameters as variables inside the function stack
        for arg in &f.arguments {
            vars.push(Variable { name : arg.name.clone(), id : next_var_id, writeable : true });
            next_var_id += 1;
        }

        let entry =
            FunctionEntry::from(f.name, if is_main { BIRL_MAIN_FUNCTION_ID } else { self.next_function_id }, f.arguments);

        if !is_main {
            self.next_function_id += 1;
        }

        self.functions.push(entry);

        self.scope = Scope::Function;

        Ok(())
    }

    pub fn add_source_string(&mut self, string : &str) -> Result<(), String> {
        unimplemented!()
    }

    pub fn add_file(&mut self, filename : &str) -> Result<(), String> {
        unimplemented!()
    }

    pub fn start_program(&mut self) -> Result<(), String> {
        unimplemented!()
    }

    pub fn start_interactive(&mut self) {
        self.vm.set_interactive();

        let entry = match self.get_entry_by_id(BIRL_GLOBAL_FUNCTION_ID) {
            Some(e) => e.clone(), // FIXME: Totally a temporary fix. fuck rust
            None => {
                println!("Erro interno : Não existe uma entry pra função global");
                return;
            },
        };

        match self.vm.prepare_function(&entry) {
            Ok(_) => {}
            Err(e) => {
                println!("Erro fatal na preparação da VM : {}", e);
                return;
            }
        }

        print!("{}\n{}\n", BIRL_COPYRIGHT, BIRL_VERSION);

        let mut line = String::new();

        loop {
            if self.vm.has_quit() {
                break;
            }

            line.clear();

            print!(">");

            self.vm.flush_stdout();

            let input = stdin();

            match input.read_line(&mut line) {
                Ok(_) => {},
                Err(e) => {
                    println!("Erro lendo da entrada padrão: {:?}", e);
                    continue;
                },
            }

            let result = match parse_line(line.as_str()) {
                Ok(r) => r,
                Err(e) => {
                    println!("Erro : {}", e);

                    continue;
                }
            };

            match result {
                ParserResult::Command(cmd) => {
                    let inst = {
                        let entry = match self.get_global_mut() {
                            Some(e) => e,
                            None => {
                                println!("Erro fatal : Não existe uma entrada pra função global no escopo atual");

                                continue;
                            }
                        };

                        let glb = None; // nevermind me

                        match Compiler::compile_command(cmd, entry, &glb) {
                            Ok(i) => i,
                            Err(e) => {
                                println!("Erro de compilação : {}", e);

                                continue;
                            }
                        }
                    };

                    for i in &inst {
                        match self.vm.run(i) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Erro de execução : {}", e);

                                continue;
                            }
                        }
                    }
                }
                _ => unimplemented!("Não implementado."),
            }
        }

        println!("Saindo...");
    }

    pub fn print_version() {
        println!("{}", BIRL_VERSION);
        println!("{}", BIRL_COPYRIGHT);
    }
}