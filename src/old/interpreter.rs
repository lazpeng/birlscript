//! Modulo do interpretador

use old::nparser::function::{Function, ExpectedParameter, parse_function_call_params};
use old::nparser::command::{Command};
use old::nparser::global::Global;
use old::nparser::kw::{SECT_GLOBAL, SECT_DEFAULT, RETVAL_VAR};
use old::nparser::AST;
use old::eval::{Value, ValueQuery, evaluate};
use std::rc::Rc;
use std::sync::Mutex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// As três diferentes possibilidades de uma comparação
pub enum Comparision {
    Equal,
    More,
    Less,
    NEqual, // Usada em Strings
}

impl Comparision {
    /// Tenta comparar dois numeros
    fn compare_num(left: f64, right: f64) -> Comparision {
        if left < right {
            Comparision::Less
        } else if left == right {
            Comparision::Equal
        } else {
            Comparision::More
        }
    }

    fn compare_str(left: String, right: String) -> Comparision {
        let (len1, len2) = (left.len(), right.len());
        if len1 < len2 {
            Comparision::Less
        } else if len1 > len2 {
            Comparision::More
        } else {
            if left == right {
                Comparision::Equal
            } else {
                Comparision::NEqual
            }
        }
    }

    /// Tenta comparar dois valores
    pub fn compare(left: Value, right: Value) -> Comparision {
        use old::eval::Value::*;
        match left {
            Str(v1) => {
                match right {
                    Str(v2) => Comparision::compare_str(v1, v2),
                    Num(v2) => Comparision::compare_str(v1, v2.to_string()),
                    NullOrEmpty => Comparision::NEqual,
                }
            }
            Num(v1) => {
                match right {
                    Num(v2) => Comparision::compare_num(v1, v2),
                    Str(v2) => {
                        Comparision::compare_num(v1,
                                    v2.parse::<f64>()
                                        .expect("Erro na conversão de String pra Número"))
                    }
                    NullOrEmpty => Comparision::NEqual,
                }
            }
            NullOrEmpty => {
                match right {
                    NullOrEmpty => Comparision::Equal,
                    _ => Comparision::NEqual,
                }
            }
        }
    }
}

/// Um simbolo. Tem um identificador e um valor. A bool serve pra definir se o valor pode ou não ser modificado
type Symbol = (String, Value, bool);

enum ReturnSignal {
    On,
    Off
}

#[derive(Debug)]
enum Variable {
    /// Uma variavel comum, nome e valor
    Sym(Symbol),
    /// Uma referencia a uma variavel, que está guardada em outro lugar.
    Ref(Rc<Mutex<Symbol>>),
}

pub struct Interpreter {
    /// Funções disponiveis pra serem executadas
    available_functions: Vec<Function>,
    /// Funções em execução
    call_stack: Vec<Section>,
    /// Variaveis definidas globalmente
    global_symbols: Vec<Rc<Mutex<Symbol>>>,
    /// Ultimo sinal lançado na vm. Reseta a cada retorno
    signal: ReturnSignal,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            available_functions: vec![],
            call_stack: vec![],
            global_symbols: vec![],
            signal: ReturnSignal::Off,
        }
    }

    fn load_ast(&mut self, ast: AST) {
        for global in ast.decl_globals {
            self.declare_global(global);
        }
        for function in ast.decl_functions {
            self.declare_function(function);
        }
    }

    fn load_asts(&mut self, asts: Vec<AST>) {
        for ast in asts {
            self.load_ast(ast);
        }
    }

    pub fn load_file(&mut self, file: &str) {
        self.load_ast(AST::load_file(file));
    }

    pub fn load_files(&mut self, files: Vec<String>) {
        if files.is_empty() { return; }
        let asts: Vec<AST> = files.iter().map(|file| AST::load_file(file)).collect();
        self.load_asts(asts);
    }

    pub fn load_sources(&mut self, sources: Vec<String>) {
        if sources.is_empty() { return; }
        let asts: Vec<AST> = sources.iter().map(|source| AST::load_string(source)).collect();
        self.load_asts(asts);
    }

    pub fn declare_global(&mut self, glb: Global) {
        let glb_sym = Rc::new( Mutex::new((glb.identifier().to_owned(), glb.value().clone(), glb.constant())) );
        self.global_symbols.push(glb_sym);
    }

    pub fn declare_function(&mut self, func: Function) {
        self.available_functions.push(func);
    }

    pub fn start(&mut self) {
        self.load_default_variables();
        if let Some(_) = self.retrieve_function(SECT_GLOBAL) {
            self.call_function(SECT_GLOBAL, vec![]);
        }
        if let Some(_) = self.retrieve_function(SECT_DEFAULT) {
            self.call_function(SECT_DEFAULT, vec![]);
        }
    }

    fn get_username(&self) -> String {
        use std::env::vars;
        for (key, value) in vars() {
            if key == "USER" || key == "USERNAME" {
                return value.to_owned();
            }
        }
        return "CUMPADE".to_owned();
    }

    fn load_default_variables(&mut self) {
        // Variaveis padrão
        // CUMPADE: nome de usuario
        // UM: inteiro 1
        // BODYBUILDER: "BAMBAM"
        let username = self.get_username();
        let glb_cumpade     = Global::from(Some("CUMPADE".to_owned()), Some(Value::Str(username)), Some(true));
        let glb_um          = Global::from(Some("UM".to_owned()), Some(Value::Num(1.0)), Some(true));
        let glb_bodybuilder = Global::from(Some("BODYBUILDER".to_owned()), Some(Value::Str("BAMBAM".to_owned())), Some(true));

        self.declare_global(glb_cumpade); self.declare_global(glb_um); self.declare_global(glb_bodybuilder);
    }

    fn exec_if(&mut self, comparision: Comparision, command: &Command) -> bool {
        let last_comparision = self.last_section().get_last_comparision();
        if comparision == last_comparision {
            self.execute_command(command);
            true
        } else if comparision == Comparision::NEqual && last_comparision != Comparision::Equal {
            self.execute_command(command);
            true
        } else {
            false
        }
    }

    fn last_section<'a>(&'a self) -> &'a Section {
        &self.call_stack[self.call_stack.len() - 1]
    }

    fn last_section_mut<'a>(&'a mut self) -> &'a mut Section {
        let len = self.call_stack.len();
        &mut self.call_stack[len - 1]
    }

    fn second_last_section_mut<'a>(&'a mut self) -> &'a mut Section {
        let len = self.call_stack.len();
        &mut self.call_stack[len - 2]
    }

    fn retrieve_function(&self, name: &str) -> Option<Function> {
        if self.available_functions.is_empty() {
            None
        } else {
            for func in &self.available_functions {
                if func.get_identifier() == name {
                    return Some(func.clone());
                }
            }
            None
        }
    }

    fn input(&self) -> String {
        use std::io::{stdin, BufRead};
        let mut buffer = String::new();
        let ref stdin = stdin();
        stdin.lock().read_line(&mut buffer).expect("Erro lendo da entrada padrão");
        buffer.trim().to_owned()
    }

    fn do_jump(&mut self, j_args: &str) {
        let (param_list, func_name) = parse_function_call_params(j_args);
        let arguments = param_list.iter().map(|elem| evaluate(elem, self.last_section()).expect("")).collect();
        self.call_function(&func_name, arguments);
    }

    fn do_return(&mut self, value: &Option<String>) {
        use std::process::exit;
        if let &Some(ref v) = value {
            let returned_value = match  evaluate(v, self.last_section()) {
                Ok(v) => v,
                Err(e) => panic!(e),
            };
            if self.call_stack.len() == 1 {
                // Somente a seção global
                match returned_value {
                    Value::Num(code) => exit(code as i32),
                    _ => {}
                }
            }
            self.second_last_section_mut().modify(RETVAL_VAR, returned_value);
        }
        // atualiza o sinal pra realizar o retorno
        self.signal = ReturnSignal::On;
        // faz pop da ultima seção
        self.call_stack.pop();
    }

    fn execute_command(&mut self, command: &Command) {
        use old::nparser::command::Command::*;
        use std::process::exit;

        match command {
            &Move(ref dest, ref expr) => {
                let expr_value = match evaluate(expr, self.last_section()) {
                    Ok(v) => v,
                    Err(e) => panic!(e),
                };
                self.last_section_mut().modify(dest, expr_value);
            },
            &Clear(ref targ) => self.last_section_mut().modify(targ, Value::NullOrEmpty),
            &Decl(ref id) => self.last_section_mut().declare( Variable::Sym((id.to_owned(), Value::NullOrEmpty, false)) ),
            &DeclWV(ref id, ref val) => {
                let value = match evaluate(val, self.last_section()) {
                    Ok(e) => e,
                    Err(e) => panic!(e),
                };
                self.last_section_mut().declare(Variable::Sym((id.to_owned(), value, false)));
            }
            &Jump(ref jarg) => self.do_jump(jarg),
            &Cmp(ref lho, ref rho) => {
                let lho_val = match evaluate(lho, self.last_section()) {
                    Ok(v) => v,
                    Err(e) => panic!(e),
                };
                let rho_val = match evaluate(rho, self.last_section()) {
                    Ok(v) => v,
                    Err(e) => panic!(e),
                };
                self.last_section_mut().set_comparision(Comparision::compare(lho_val, rho_val));
            },
            &CmpEq    (ref cmd) => { self.exec_if(Comparision::Equal, cmd); },
            &CmpNEq   (ref cmd) => { self.exec_if(Comparision::NEqual, cmd); },
            &CmpLess  (ref cmd) => { self.exec_if(Comparision::Less, cmd); },
            &CmpMore  (ref cmd) => { self.exec_if(Comparision::More, cmd); },
            &CmpLessEq(ref cmd) => {
                if !self.exec_if(Comparision::Less, cmd) {
                    self.exec_if(Comparision::Equal, cmd);
                }
            },
            &CmpMoreEq(ref cmd) => {
                if !self.exec_if(Comparision::More, cmd) {
                    self.exec_if(Comparision::Equal, cmd);
                }
            },
            &Print(ref what) => {
                if !what.is_empty() {
                    for w in what {
                        let val = match evaluate(w, self.last_section_mut()) {
                            Ok(v) => v,
                            Err(e) => panic!(e),
                        };
                        print!("{}", val);
                    }
                }
            },
            &Println(ref what) => {
                if !what.is_empty() {
                    for w in what {
                        let val = match evaluate(w, self.last_section_mut()) {
                            Ok(v) => v,
                            Err(e) => panic!(e),
                        };
                        print!("{}", val);
                    }
                }
                println!();
            },
            &Quit(ref code) => exit(code.parse().unwrap_or(0)),
            &Return(ref val) => self.do_return(val),
            &Input(ref dest) => {
                let inputted = self.input();
                self.last_section_mut().modify(dest, Value::Str(inputted));
            },
            &InputUpper(ref dest) => {
                let inputted = self.input().to_uppercase();
                self.last_section_mut().modify(dest, Value::Str(inputted));
            },
        }
    }

    pub fn call_function(&mut self, function_name: &str, arguments: Vec<Value>) {
        let function = self.retrieve_function(function_name).expect(&format!("Erro ao encontrar a função {}", function_name));

        if !ExpectedParameter::matches_with(function.get_parameters(), &arguments) {
            panic!("Função chamada com lista errada de argumentos.");
        }

        let mut new_section = Section {
            stack: vec![],
            comparision: Comparision::NEqual
        };

        // Cria a variavel constante que contem o nome da seção
        new_section.declare(Variable::Sym( ("JAULA".to_owned(), Value::Str(function.get_identifier().to_owned()), true) ));

        new_section.declare(Variable::Sym( (RETVAL_VAR.to_owned(), Value::NullOrEmpty, false)));

        // Empurra referencias aos globais pra nova seção
        for global in &self.global_symbols {
            new_section.declare(Variable::Ref(global.clone()));
        }

        let function_obj_params = function.get_parameters();

        // Empurra os argumentos passados
        for index in 0..arguments.len() {
            let (argument_value, argument_id) = (arguments[index].clone(), function_obj_params[index].id.to_owned());
            new_section.declare(Variable::Sym((argument_id, argument_value, false)));
        }

        self.call_stack.push(new_section);

        // Executa os comandos
        for cmd in function.get_commands() {
            self.execute_command(cmd);
            if let ReturnSignal::On = self.signal {
                // reset
                self.signal = ReturnSignal::Off;
                break;
            }
        }
    }
}

#[derive(Debug)]
struct Section {
    stack: Vec<Variable>,
    comparision: Comparision,
}

impl ValueQuery for Section {
    fn query(&self, id: &str) -> Option<Value> {
        let id = id.trim();
        if self.stack.is_empty() { None }
        else {
            let mut result = None;

            for var in &self.stack {
                match var {
                    &Variable::Sym(ref s) => {
                        if s.0.trim() == id {
                            result = Some(s.1.clone());
                            break;
                        }
                    }
                    &Variable::Ref(ref r) => {
                        let ref mut inner = r.lock().expect("Erro reservando acesso ao mutex.");
                        if inner.0.trim() == id {
                            result = Some(inner.1.clone());
                            break;
                        }
                    }
                }
            }

            if let None = result {
                println!("result none: stack: {:?}", self.stack);
            };

            result
        }
    }
}

impl Section {
    fn declare(&mut self, var: Variable) {
        // não checa se a variavel ja existe. Sempre a primeira a ser declarada é reconhecida
        self.stack.push(var);
    }

    fn set_comparision(&mut self, cmp: Comparision) {
        self.comparision = cmp;
    }

    fn modify(&mut self, id: &str, value: Value) {
        use std::ops::DerefMut;
        let id = id.trim();
        for var in &mut self.stack {
            match var {
                &mut Variable::Sym(ref mut s) => {
                    let (vid, mut val, constant) = (&s.0, &mut s.1, &s.2);
                    if vid == id {
                        if *constant {
                            panic!("Erro ao tentar modificar simbolo constante");
                        }
                        *val = value;
                        break;
                    }
                }
                &mut Variable::Ref(ref mut r) => {
                    let ref mut inner = r.lock().expect("Erro ao adquirir trava do Mutex");
                    if inner.0 == id {
                        if inner.2 {
                            panic!("Erro ao tentar modificar simbolo constante");
                        }
                        let value_mut_ref: &mut Value = &mut inner.deref_mut().1;
                        *value_mut_ref = value;
                        break;
                    }
                }
            }
        }
    }

    fn get_last_comparision(&self) -> Comparision {
        self.comparision
    }
}
