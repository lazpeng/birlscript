//! Responsável pela execução do programa
#![allow(dead_code)]

use parser;
use value;

/// Endereço das variáveis
type Address = i16;

/// Variavel que tem um nome e um valor
#[derive(Clone)]
pub struct Variable {
    /// Identificador da String
    pub id: String,
    /// Valor da variavel
    pub value: value::Value,
    /// Endereço da variavel
    address: Address,
    /// Se a variavel é constante ou não
    constant: bool,
}

impl Variable {
    fn new() -> Variable {
        Variable {
            id: String::new(),
            value: value::Value::Number(0.0),
            address: -1,
            constant: true,
        }
    }

    fn from(vid: String, val: value::Value, is_const: bool) -> Variable {
        Variable {
            id: vid,
            value: val,
            address: -1,
            constant: is_const,
        }
    }
}

/// Opções que podem ser passadas ao ambiente
pub struct EnvironmentOptions {
    /// Nome da seção padrão na hora de inicializar o programa
    default_section: String,
    /// Se o interpretador deve se comportar de forma verbosa
    verbose: bool,
}

/// Implementação
impl EnvironmentOptions {
    /// Constre um novo base
    pub fn new() -> EnvironmentOptions {
        EnvironmentOptions {
            default_section: String::from("SHOW"),
            verbose: false,
        }
    }

    /// Define a variavel verbose
    pub fn set_verbose(&mut self, value: bool) {
        self.verbose = value;
    }

    /// Define a variavel default_section
    pub fn set_default_section(&mut self, value: String) {
        self.default_section = value;
    }
}

/// É o ambiente onde rodam os scripts BIRL
pub struct Environment {
    /// Pilha de variaveis do ambiente
    variables: Vec<Variable>,
    /// Coleção de seções para serem executadas
    sections: Vec<parser::Section>,
    /// Opções
    options: EnvironmentOptions,
}

impl Environment {
    /// Cria um novo ambiente
    pub fn new(opts: EnvironmentOptions) -> Environment {
        Environment {
            variables: vec![],
            sections: vec![],
            options: opts,
        }
    }

    /// Declara uma variavel e retorna seu endereço
    fn declare_var(&mut self, var: Variable) -> i16 {
        let addr: i16 = self.variables.len() as i16;
        let mut vcpy = var.clone();
        vcpy.address = addr;
        self.variables.push(vcpy);
        addr
    }

    /// Interpreta uma unidade sem executá-la
    pub fn interpret(&mut self, file: parser::Unit) {
        for const_var in file.consts {
            let var = Variable {
                id: const_var.identifier,
                value: value::parse_expr(&const_var.value, self),
                address: 0,
                constant: true,
            };
            self.declare_var(var);
        }
        for sect in file.sects {
            self.sections.push(sect);
        }
    }

    /// Pega uma variavel do ambiente
    pub fn get_var(&self, name: &str) -> Option<Variable> {
        if self.variables.len() <= 0 {
            None
        } else {
            let mut ret: Option<Variable> = None;
            for var in &self.variables {
                if var.id == name {
                    ret = Some(var.clone());
                    break;
                }
            }
            ret
        }
    }

    // Inicio da implementação dos comandos

    /// Implementação do Print
    fn command_print(&mut self, message: value::Value) {
        print!("{}", message);
    }

    /// Implementação do Println
    fn command_println(&mut self, message: value::Value) {
        println!("{}", message);
    }

    /// Executa um comando
    fn execute_command(&mut self, cmd: parser::Command) {
        use parser::Command;
        match cmd {
            Command::Print(msg) => {
                let msg = value::parse_expr(&msg, self);
                self.command_print(msg);
            }
            Command::Println(msg) => {
                let msg = value::parse_expr(&msg, self);
                self.command_println(msg);
            }
            _ => {}
        }
    }

    /// Executa uma seção, se preciso, recursivamente
    fn execute_section(&mut self, sect_name: &str) {
        let mut section = parser::Section::new();
        let mut found = false;
        for sect in &self.sections {
            if sect.name == sect_name {
                section = sect.clone();
                found = true;
                break;
            }
        }
        if !found {
            panic!("Erro: Seção não encontrada: \"{}\".", sect_name);
        } else {
            for cmd in section.lines {
                self.execute_command(cmd);
            }
        }
    }

    /// Configura as variaveis basicas
    fn init_variables(&mut self) {
        use std::env;
        let var_names = vec!["CUMPADE", "UM", "BODYBUILDER"];
        let mut var_cumpade: String = String::from("\"") + &env::var("USER").unwrap();
        var_cumpade.push('\"');
        let var_values = vec![value::Value::Str(var_cumpade),
                              value::Value::Number(1.0),
                              value::Value::Str(String::from("BAMBAM"))];
        for i in 0..var_names.len() {
            let (name, val) = (var_names[i], var_values[i].clone());
            let var = Variable::from(name.to_string(), val, true);
            self.declare_var(var);
        }
    }

    /// Executa a seção padrão
    pub fn start_program(&mut self) {
        let name = self.options.default_section.clone();
        self.init_variables();
        self.execute_section(&name);
    }
}
