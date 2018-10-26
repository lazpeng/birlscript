//! Hosts the runtime for the birlscript language

// BIG TODO : Add default variables

use vm::{ Instruction, VirtualMachine, ExecutionStatus };
use parser::{ parse_line, FunctionParameter, ParserResult, IntegerType, FunctionDeclaration };
use compiler::{ Compiler, Variable, CompilerHint };

use std::io::{ BufRead, BufReader };
use std::fs::File;

pub const BIRL_COPYRIGHT : &'static str 
    = "© 2016 - 2018 Rafael Rodrigues Nakano <lazpeng@gmail.com>";
pub const BIRL_VERSION : &'static str 
    = "BirlScript v2.0.0-alpha";
pub const BIRL_MAIN_FUNCTION : &str 
    = "SHOW";

pub const BIRL_MAIN_FUNCTION_ID     : u64 = 1;
pub const BIRL_GLOBAL_FUNCTION_ID   : u64 = 0;
pub const BIRL_RET_VAL_VAR_ID       : u64 = 0;

#[derive(Debug, Clone)]
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
            vars : vec![Variable {
                name : "TREZE".to_owned(),
                id : BIRL_RET_VAL_VAR_ID,
                writeable : true
            }],
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

pub enum RawValue {
    Text(String),
    Integer(IntegerType),
    Number(f64)
}

pub struct Context {
    vm : VirtualMachine,
    functions : Vec<FunctionEntry>,
    scope : Scope,
    has_main : bool,
    next_function_id : u64,
    global_scope : Vec<ScopeManager>,
    function_scope : Vec<ScopeManager>,
    last_function_id : u64,
}

struct ScopeManager {
    ids : Vec<u64>
}
impl ScopeManager {
    fn empty() -> ScopeManager {
        ScopeManager {
            ids : vec![]
        }
    }

    fn at_end(self, func : &mut FunctionEntry) {
        for id in self.ids {
            for i in 0..func.vars.len() {
                if func.vars[i].id == id {
                    func.vars.remove(i);
                    break;
                }
            }
        }
    }
}

impl Context {
    fn new_global() -> FunctionEntry {
        FunctionEntry::from("__global__".to_owned(), BIRL_GLOBAL_FUNCTION_ID, vec![])
    }

    pub fn new() -> Context {
        Context {
            vm : VirtualMachine::new(),
            functions : vec![Context::new_global()],
            scope : Scope::Global,
            has_main : false,
            next_function_id : 2,
            global_scope : vec![ScopeManager::empty()],
            function_scope : vec![],
            last_function_id : 0,
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

    fn add_function(&mut self, f : FunctionDeclaration) -> Result<u64, String> {
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
            let tmp = self.next_function_id;

            self.next_function_id += 1;

            tmp
        };
        let mut entry = FunctionEntry::from(f.name, id, f.arguments.clone());

        // Register all parameters as variables inside the function stack
        for arg in f.arguments {
            match entry.add_var(arg.name, true) {
                Ok(_) => {}
                Err(e) => return Err(e)
            }
        }

        self.functions.push(entry);
        self.scope = Scope::Function;
        self.last_function_id = id;

        Ok(id)
    }

    fn process_line(&mut self, line : &str) -> Result<(), String> {
        let mut instructions = vec![];

        let result = match parse_line(line) {
            Ok(r) => r,
            Err(e) => return Err(e)
        };

        match result {
            ParserResult::Command(cmd) => {
                let hint = {
                    let funcs = &self.functions;

                    let (global, current) = match self.scope {
                        Scope::Global => {
                            match self.get_entry_by_id(BIRL_GLOBAL_FUNCTION_ID) {
                                Some(g) => (None, g),
                                None => return Err(format!("Erro fatal : Função global não registrada"))
                            }
                        }
                        Scope::Function => {
                            let global = match self.get_entry_by_id(BIRL_GLOBAL_FUNCTION_ID) {
                                Some(g) => Some(g),
                                None => return Err(format!("Erro fatal : Função global não registrada"))
                            };

                            let id = self.last_function_id;

                            let func = match self.get_entry_by_id(id) {
                                Some(f) => f,
                                None => return Err(format!("Não foi encontrada função com ID {}", id))
                            };

                            (global, func)
                        }
                    };

                    match Compiler::compile_command(cmd, current, &global, funcs, &mut instructions) {
                        Ok(hint) => hint,
                        Err(e) => return Err(e)
                    }
                };

                if let Some(hint) = hint {
                    match hint {
                        CompilerHint::DeclareVar(var) => {
                            let id = {
                                let entry = match self.scope {
                                    Scope::Global => {
                                        match self.get_entry_by_id_mut(BIRL_GLOBAL_FUNCTION_ID) {
                                            Some(f) => f,
                                            None => return Err("Erro fatal : Nenhuma função global".to_owned())
                                        }
                                    }
                                    Scope::Function => {
                                        let id = self.last_function_id;
                                        match self.get_entry_by_id_mut(id) {
                                            Some(f) => f,
                                            None => return Err(format!("Erro fatal : Nenhuma função com ID {}", id)),
                                        }
                                    }
                                };

                                match entry.add_var(var.name, var.writeable) {
                                    Ok(id) => id,
                                    Err(e) => return Err(e)
                                }
                            };

                            let scope = match self.scope {
                                Scope::Global => {
                                    if self.global_scope.is_empty() {
                                        return Err("Erro fatal : Scopes tá vazio".to_owned());
                                    }

                                    self.global_scope.last_mut().unwrap()
                                }
                                Scope::Function => {
                                    if self.function_scope.is_empty() {
                                        return Err("Erro fatal : Scopes tá vazio".to_owned());
                                    }

                                    self.function_scope.last_mut().unwrap()
                                }
                            };

                            scope.ids.push(id);
                        }
                        CompilerHint::ScopeStart => {
                            let scope = match self.scope {
                                Scope::Global => &mut self.global_scope,
                                Scope::Function => &mut self.function_scope,
                            };

                            scope.push(ScopeManager::empty());
                        }
                        CompilerHint::ScopeEnd => {
                            let scope = {
                                let scope = match self.scope {
                                    Scope::Global => &mut self.global_scope,
                                    Scope::Function => &mut self.function_scope,
                                };

                                if scope.is_empty() {
                                    return Err("Erro fatal : Scope vazio".to_owned());
                                }

                                let index = scope.len() - 1;

                                scope.remove(index)
                            };

                            let func = match self.scope {
                                Scope::Global => {
                                    match self.get_entry_by_id_mut(BIRL_GLOBAL_FUNCTION_ID) {
                                        Some(f) => f,
                                        None => return Err("Erro fatal : Não encontrada função global".to_owned()),
                                    }
                                }
                                Scope::Function => {
                                    let id = self.last_function_id;

                                    match self.get_entry_by_id_mut(id) {
                                        Some(f) => f,
                                        None => return Err(format!("Erro fatal : Nenhuma função com ID {}", id))
                                    }
                                }
                            };

                            scope.at_end(func);
                        }
                    }
                }

                let body = match self.scope {
                    Scope::Function => {
                        let id = self.last_function_id;
                        match self.get_entry_by_id_mut(id) {
                            Some(f) => &mut f.body,
                            None => return Err(format!("Erro fatal : Nenhuma função com ID {}", id)),
                        }
                    }
                    Scope::Global => {
                        match self.get_entry_by_id_mut(BIRL_GLOBAL_FUNCTION_ID) {
                            Some(f) => &mut f.body,
                            None => return Err(format!("Erro fatal : Nenhuma função global"))
                        }
                    }
                };

                for i in instructions {
                    body.push(i);
                }
            }
            ParserResult::FunctionEnd => {
                if self.scope != Scope::Function {
                    return Err("Erro : Fim de função fora de uma função".to_owned());
                }

                if self.function_scope.len() > 1 {
                    return Err("Erro : Feche todos os scopes antes de terminar a função".to_owned());
                } else if self.function_scope.is_empty() {
                    return Err("Erro fatal : Scopes tá vazio".to_owned());
                }

                let mut last_scope = self.function_scope.remove(0);

                let id = self.last_function_id;

                match self.get_entry_by_id_mut(id) {
                    Some(f) => last_scope.at_end(f),
                    None => return Err(format!("Erro fatal : Nenhuma função com ID {}", id))
                }

                self.scope = Scope::Global;
            }
            ParserResult::FunctionStart(func) => {
                if self.scope != Scope::Global {
                    return Err("Erro : Declaração de função fora do escopo global".to_owned());
                }

                match self.add_function(func) {
                    Ok(_) => {},
                    Err(e) => return Err(e)
                }

                self.scope = Scope::Function;

                self.function_scope.push(ScopeManager::empty());
            }
            ParserResult::Nothing => return Ok(())
        }

        Ok(())
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

    pub fn call_function_by_id(&mut self, id : u64, mut args : Vec<RawValue>) -> Result<(), String> {
        let mut instructions = vec![];

        for f in &self.functions {
            if f.id == id {

                if f.params.len() != args.len() {
                    return Err(format!("A função {} espera {} argumentos, mas {} foram passados",
                                       f.name, f.params.len(), args.len()));
                }

                instructions.push(Instruction::MakeNewFrame(id));

                for i in 0..args.len() {
                    let exp = f.params[i].kind;

                    let mut eid = None;

                    let arg_name = f.params[i].name.as_str();

                    for v in &f.vars {
                        if v.name == arg_name {
                            eid = Some(v.id);

                            break;
                        }
                    }

                    if let None = eid {
                        return Err(format!("Erro interno : O argumento {} não tá registrado como variável", arg_name));
                    }

                    let val = args.remove(0);

                    match val {
                        RawValue::Text(t) => instructions.push(Instruction::PushMainStr(t)),
                        RawValue::Number(n) => instructions.push(Instruction::PushMainNum(n)),
                        RawValue::Integer(i) => instructions.push(Instruction::PushMainInt(i)),
                    }

                    instructions.push(Instruction::AssertMainTopTypeCompatible(exp));

                    instructions.push(Instruction::WriteToVarWithId(eid.unwrap()));
                }

                instructions.push(Instruction::SetLastFrameReady);
            }
        }

        if instructions.is_empty() {
            return Err(format!("Não encontrada a função com ID {}", id));
        }

        for i in instructions {
            match self.vm.run(&i) {
                Ok(_) => {}
                Err(e) => return Err(e)
            }
        }

        Ok(())
    }

    pub fn call_function_by_name(&mut self, name : &str, args : Vec<RawValue>) -> Result<(), String> {
        let mut id = None;

        for f in &self.functions {
            if f.name == name {
                id = Some(f.id);

                break;
            }
        }

        if let Some(id) = id {
            self.call_function_by_id(id, args)
        } else {
            Err(format!("Função {} não encontrada.", name))
        }
    }

    fn execute_next_instruction(&mut self) -> Result<ExecutionStatus, String> {

        let pc = match self.vm.get_current_pc() {
            Some(p) => p,
            None => return Err("Erro recebendo PC : Nenhuma função em execução".to_owned())
        };

        let id = match self.vm.get_current_id() {
            Some(id) => id,
            None => return Err("Erro recebendo ID atual : Nenhuma função em execução".to_owned())
        };

        let instruction = match self.get_entry_by_id(id) {
            Some(e) => {
                let body_len = e.body.len();

                if pc >= body_len {
                    Instruction::Return
                } else {
                    // FIXME: There's probably a way to take this as reference. later tho
                    e.body[pc].clone()
                }
            }
            None => return Err(format!("Nenhuma função com ID {}", id))
        };

        match self.vm.increment_pc() {
            Ok(_) => {}
            Err(e) => return Err(e)
        }

        let status = match self.vm.run(&instruction) {
            Ok(status) => status,
            Err(e) => return Err(e)
        };

        Ok(status)
    }

    pub fn start_program(&mut self) -> Result<(), String> {
        match self.call_function_by_id(BIRL_GLOBAL_FUNCTION_ID, vec![]) {
            Ok(_) => {}
            Err(e) => return Err(e)
        }

        loop {
            match self.execute_next_instruction() {
                Ok(ExecutionStatus::Normal) => {}
                Ok(ExecutionStatus::Returned) => {},
                Ok(ExecutionStatus::Quit) => break,
                Err(e) => return Err(e)
            }
        }

        self.vm.unset_quit();

        if self.has_main {
            match self.call_function_by_name(BIRL_MAIN_FUNCTION, vec![]) {
                Ok(_) => {}
                Err(e) => return Err(e)
            }

            loop {
                match self.execute_next_instruction() {
                    Ok(ExecutionStatus::Normal) => {}
                    Ok(ExecutionStatus::Returned) => {}
                    Ok(ExecutionStatus::Quit) => break,
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
