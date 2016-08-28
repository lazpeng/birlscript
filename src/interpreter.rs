//! Responsável pela execução do programa

use parser;
use value;
use error;

/// Variavel que tem um nome e um valor
#[derive(Clone)]
pub struct Variable {
    /// Identificador da String
    pub id: String,
    /// Valor da variavel
    pub value: value::Value,
    /// Se a variavel é constante ou não
    constant: bool,
}

impl Variable {

    /// Cria uma variavel com uma serie de informações
    fn from(vid: String, val: value::Value, is_const: bool) -> Variable {
        Variable {
            id: vid,
            value: val,
            constant: is_const,
        }
    }
}

/// Representação de uma comparação
pub enum Comparision {
    /// Igual
    Equals,
    /// Menor
    Less,
    /// Maior
    More,
    /// Nenhum dos anteriores (valor inicial)
    None,
}

/// Compara duas strings
fn compare_str(str1: value::Value, str2: value::Value) -> Comparision {
    if let value::Value::Str(value1) = str1 {
        if let value::Value::Str(value2) = str2 {
            let ret: Comparision = if value1 == value2 {
                Comparision::Equals
            } else if value1.len() < value2.len() {
                Comparision::Less
            }
            else if value1 != value2 {
                Comparision::None
            } else {
                Comparision::More
            };
            ret
        } else {
            error::abort("Comparação de string com outro tipo");
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

/// Compara dois numeros
fn compare_num(num1: value::Value, num2: value::Value) -> Comparision {
    if let value::Value::Number(value1) = num1 {
        if let value::Value::Number(value2) = num2 {
            let ret: Comparision = if value1 == value2 {
                Comparision::Equals
            } else if value1 < value2 {
                Comparision::Less
            } else {
                Comparision::More
            };
            ret
        } else {
            error::abort("Comparação de caractere com outro tipo");
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

/// Compara dois caracteres
fn compare_char(c1: value::Value, c2: value::Value) -> Comparision {
    if let value::Value::Char(value1) = c1 {
        if let value::Value::Char(value2) = c2 {
            let ret: Comparision = if value1 == value2 {
                Comparision::Equals
            } else if value1 < value2 {
                Comparision::Less
            } else {
                Comparision::More
            };
            ret
        } else {
            error::abort("Comparação de caractere com outro tipo");
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

/// Compara dois valores e retorna um resultado
fn compare(val1: value::Value, val2: value::Value) -> Comparision {
    match val1 {
        value::Value::Str(_) => compare_str(val1, val2),
        value::Value::Number(_) => compare_num(val1, val2),
        value::Value::Char(_) => compare_char(val1, val2),
    }
}

/// Retorna o input da entrada padrão
fn get_input() -> String {
    use std::io;
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {},
        Err(e) => error::abort(&format!("Erro ao ler a entrada padrão! \"{}\"", e)),
    }
    buffer.trim().to_string()
}

/// É o ambiente onde rodam os scripts BIRL
pub struct Environment {
    /// Pilha de variaveis do ambiente
    variables: Vec<Variable>,
    /// Coleção de seções para serem executadas
    sections: Vec<parser::Section>,
    /// Ponto de entrada para o programa
    entry: String,
    /// O resultado da ultima Comparação
    last_cmp: Comparision
}

impl Environment {
    /// Cria um novo ambiente
    pub fn new(entry_point: String) -> Environment {
        Environment {
            variables: vec![],
            sections: vec![],
            entry: entry_point,
            last_cmp: Comparision::None,
        }
    }

    /// Declara uma variavel e retorna seu endereço
    fn declare_var(&mut self, var: Variable) {
        if self.variables.len() > 0 {
            for v in &self.variables {
                if v.id == var.id {
                    error::abort(&format!("Variavel \"{}\" já declarada", var.id));
                }
            }
        }
        self.variables.push(var);
    }

    /// Interpreta uma unidade sem executá-la
    pub fn interpret(&mut self, file: parser::Unit) {
        for const_var in file.consts {
            let var = Variable::from(const_var.identifier, value::parse_expr(&const_var.value, self), true);
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

    /// Modifica o valor de uma variavel
    pub fn mod_var(&mut self, var: &str, newval: value::Value) {
        if self.variables.len() < 1 {
            error::abort("Nenhuma variavel declarada!");
        }
        let (mut index, mut found) = (0, false);
        loop {
            if index >= self.variables.len() {
                break;
            }
            let ref mut v = self.variables[index];
            if v.id == var {
                v.value = newval;
                found = true;
                break;
            }
            index += 1;
        }
        if !found {
            error::abort(&format!("Variavel não encontrada: \"{}\"", var));
        }
    }

    // Inicio da implementação dos comandos

    /// Seta o valor de uma variável
    fn command_move(&mut self, target: String, val: value::Value) {
        self.mod_var(&target, val);
    }

    /// Limpa o valor de uma variavel
    fn command_clear(&mut self, target: String) {
        self.mod_var(&target, value::Value::Number(0.0));
    }

    /// Declara uma variavel com o valor padrão
    fn command_decl(&mut self, name: String) {
        let var = Variable::from(name, value::Value::Number(0.0), false);
        self.declare_var(var);
    }

    /// Declara uma variavel com um valor padrão
    fn command_declwv(&mut self, name: String, val: value::Value) {
        let var = Variable::from(name, val, false);
        self.declare_var(var);
    }

    /// Passa a execução para outra seção
    fn command_jump(&mut self, section: String) {
        self.execute_section(&section);
    }

    /// Compara dois valores
    fn command_cmp(&mut self, val1: value::Value, val2: value::Value) {
        self.last_cmp = compare(val1, val2);
    }

    /// Executa uma seção caso comparação de equals
    fn command_cmp_eq(&mut self, sect: String) {
        if let Comparision::Equals = self.last_cmp {
            self.execute_section(&sect);
        }
    }

    /// Executa uma seção caso comparação dê not equals
    fn command_cmp_neq(&mut self, sect: String) {
        if let Comparision::More = self.last_cmp {
            self.execute_section(&sect);
        } else if let Comparision::None = self.last_cmp {
            self.execute_section(&sect);   
        }else if let Comparision::Less = self.last_cmp {
            self.execute_section(&sect);
        }
    }

    /// Menos/Menor
    fn command_cmp_less(&mut self, sect: String) {
        if let Comparision::Less = self.last_cmp {
            self.execute_section(&sect);
        }
    }

    /// Menos/Menor ou igual
    fn command_cmp_lesseq(&mut self, sect: String) {
        if let Comparision::Less = self.last_cmp {
            self.execute_section(&sect);
        } else if let Comparision::Equals = self.last_cmp {
            self.execute_section(&sect);
        }
    }

    /// Maior
    fn command_cmp_more(&mut self, sect: String) {
        if let Comparision::More = self.last_cmp {
            self.execute_section(&sect);
        }
    }

    /// Maior ou igual
    fn command_cmp_moreeq(&mut self, sect: String) {
        if let Comparision::More = self.last_cmp {
            self.execute_section(&sect);
        } else if let Comparision::Equals = self.last_cmp {
            self.execute_section(&sect);
        }
    }

    /// Implementação do Print
    fn command_print(&mut self, messages: Vec<String>) {
        for message in messages {
            print!("{}", value::parse_expr(&message, self));
        }
    }

    /// Implementação do Println
    fn command_println(&mut self, messages: Vec<String>) {
        if messages.len() > 0 {
            for msg in messages {
                print!("{}", value::parse_expr(&msg, self));
            }
        }
        println!("");
    }

    /// Quit
    fn command_quit(&mut self) {
        use std::process;
        process::exit(0);
    }

    /// Input
    fn command_input(&mut self, var: String) {
        let input = get_input();
        let mut res = String::from("\"");
        res.push_str(&input);
        res.push('\"');
        self.mod_var(&var, value::Value::Str(res));
    }

    /// Input upper
    fn command_input_upper(&mut self, var: String) {
        let input = get_input().to_uppercase();
        let mut res = String::from("\"");
        res.push_str(&input);
        res.push('\"');
        self.mod_var(&var, value::Value::Str(res));
    }

    /// Executa um comando
    fn execute_command(&mut self, cmd: parser::Command) {
        use parser::Command;
        match cmd {
            Command::Move(trg, val) => {
                let val = value::parse_expr(&val, self);
                self.command_move(trg, val);
            }
            Command::Clear(trg) => self.command_clear(trg),
            Command::Decl(trg) => self.command_decl(trg),
            Command::DeclWV(trg, val) => {
                let val = value::parse_expr(&val, self);
                self.command_declwv(trg, val);
            }
            Command::Jump(sect) => self.command_jump(sect),
            Command::Cmp(val1, val2) => {
                let val1 = value::parse_expr(&val1, self);
                let val2 = value::parse_expr(&val2, self);
                self.command_cmp(val1, val2);
            }
            Command::CmpEq(sect) => self.command_cmp_eq(sect),
            Command::CmpNEq(sect) => self.command_cmp_neq(sect),
            Command::CmpLess(s) => self.command_cmp_less(s),
            Command::CmpLessEq(s) => self.command_cmp_lesseq(s),
            Command::CmpMore(s) => self.command_cmp_more(s),
            Command::CmpMoreEq(s) => self.command_cmp_moreeq(s),
            Command::Print(msg) => {
                self.command_print(msg);
            }
            Command::Println(msg) => {
                self.command_println(msg);
            }
            Command::Quit => {
                self.command_quit();
            }
            Command::Input(var) => {
                self.command_input(var);
            }
            Command::InputUpper(var) => {
                self.command_input_upper(var);
            }
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
            error::abort(&format!("Seção não encontrada: \"{}\".", sect_name));
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
        let user_varenv = if cfg!(windows) {
            // No windows, a variavel de ambiente que contem o nome de usuario é diferente
            "USERNAME"
        } else {
            "USER"
        };
        let mut var_cumpade: String = String::from("\"") + &(match env::var(user_varenv) {
            Ok(usr) => usr,
            Err(_) => var_names[0].to_string(), // CUMPADE
        });
        var_cumpade.push('\"');
        let var_values = vec![value::Value::Str(var_cumpade.to_uppercase()),
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
        self.init_variables();
        let entry = self.entry.clone();
        self.execute_section(&entry);
    }
}
