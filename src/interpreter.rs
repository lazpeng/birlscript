//! Responsável pela execução do programa

use parser;
use value;

/// Nome da JAULA padrão
pub const BIRL_MAIN: &'static str = "SHOW";

/// Variavel que tem um nome e um valor
#[derive(Clone)]
pub struct Variable {
    /// Identificador da String
    pub id: String,
    /// Valor da variavel
    pub value: value::Value,
    /// Se a variavel pode ser modificada, no caso da conversão de um global
    pub is_const: bool,
}

impl Variable {
    /// Cria uma variavel com uma serie de informações
    fn from(vid: String, val: value::Value) -> Variable {
        Variable {
            id: vid,
            value: val,
            is_const: false,
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
fn compare_str(val: String, str2: value::Value) -> Comparision {
    if let value::Value::Str(value2) = str2 {
        let ret: Comparision = if val == *value2 {
            Comparision::Equals
        } else if val != *value2 {
            Comparision::None
        } else if val.len() < value2.len() {
            Comparision::Less
        } else {
            Comparision::More
        };
        ret
    } else {
        abort!("Comparação de string com outro tipo")
    }
}

/// Compara dois numeros
fn compare_num(value: f64, num2: value::Value) -> Comparision {
    if let value::Value::Number(value2) = num2 {
        let ret: Comparision = if value == value2 {
            Comparision::Equals
        } else if value < value2 {
            Comparision::Less
        } else {
            Comparision::More
        };
        ret
    } else {
        abort!("Comparação de caractere com outro tipo")
    }
}

/// Compara dois valores e retorna um resultado
fn compare(val1: value::Value, val2: value::Value) -> Comparision {
    match val1 {
        value::Value::Str(val) => compare_str(*val, val2),
        value::Value::Number(val) => compare_num(val, val2),
    }
}

/// Retorna o input da entrada padrão
fn get_input() -> String {
    use std::io;
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {}
        Err(e) => abort!("Erro ao ler a entrada padrão! \"{}\"", e),
    }
    buffer.trim().to_string()
}

fn get_num_input() -> f64 {
    let inp = get_input();
    match inp.parse() {
        Ok(x) => x,
        Err(_) => 0.0,
    }
}

/// Ambiente em que são executados os comandos
pub struct SectionEnvironment {
    /// Variaveis alocadas nessa seção
    vars: Vec<Variable>,
    /// Ultimo sinal recebido
    last_sig: Option<CommandSignal>,
}

impl SectionEnvironment {
    /// Retorna o contexto primario de seção, a seção global
    pub fn root() -> SectionEnvironment {
        SectionEnvironment::new()
    }

    /// Retorna um contexto com um nome especifico
    pub fn new() -> SectionEnvironment {
        SectionEnvironment { vars: vec![], last_sig: None }
    }
}

/// É o ambiente onde rodam os scripts BIRL
pub struct Environment {
    /// Pilha de seções
    sectenvs: Vec<SectionEnvironment>,
    /// Coleção de seções para serem executadas
    sections: Vec<parser::Section>,
    /// Comandos globais.birl a serem executados
    glb_cmds: Vec<parser::Command>,
    /// Globais
    glbs: Vec<parser::Global>,
    /// Ponto de entrada para o programa
    entry: String,
    /// O resultado da ultima Comparação
    last_cmp: Comparision,
}

/// Verifica se os tipos dos parametros passados e dos parametros declarados coincidem
fn received_params_match(sect_name: &str,
                         expected: &Vec<parser::ExpectedParameter>,
                         received: &Vec<value::ValueType>)
                         -> bool {
    if expected.len() != received.len() {
        abort!("Numero incorreto de parametros passados! \"{}\" espera {} parâmetros, porém {} \
                foram passados.",
               sect_name,
               expected.len(),
               received.len())
    } else {
        let mut res = true;
        let mut index = 0;
        loop {
            if index >= expected.len() {
                break;
            }
            if !expected[index].tp.equals(received[index].clone()) {
                res = false;
                break;
            }
            index += 1;
        }
        res
    }
}

#[derive(Clone)]
/// O tipo de sinal retornado apos a execução de um valor, como de finalizar uma seção ou esperar alguns segundos
enum CommandSignal {
    /// Quita do programa com um codigo de erro
    Quit(i32),
    /// Retorna da seção atual
    Return, // Return não possui valor pois é responsabilidade da seção colocar o valor de retorno na penultima sectenv
    /// Espera por x segundos a thread atual
    Wait(u64),
}

impl Environment {
    /// Retorna o ultimo sectenv
    fn last_sectenv<'a>(&'a mut self) -> &'a mut SectionEnvironment {
        if self.sectenvs.len() == 0 {
            unreachable!()
        } else {
            let len = self.sectenvs.len();
            &mut self.sectenvs[len - 1]
        }
    }

    /// Cria um novo ambiente
    pub fn new(entry_point: String) -> Environment {
        Environment {
            sectenvs: vec![SectionEnvironment::root()],
            sections: vec![],
            glb_cmds: vec![],
            glbs: vec![],
            entry: entry_point,
            last_cmp: Comparision::None,
        }
    }

    /// Declara uma variavel e retorna seu endereço
    fn declare_var(&mut self, var: Variable) {
        let last_sectenv = self.sectenvs.len() - 1;
        self.declare_var_in(last_sectenv, var);
    }

    /// Declara uma variavel na seção passada. O numero deve ser o index da seção nos sectenvs
    fn declare_var_in(&mut self, sect_num: usize, var: Variable) {
        if self.sectenvs[sect_num].vars.len() >= sect_num - 1 {
            for v in &self.sectenvs[sect_num].vars {
                if v.id == var.id {
                    abort!("Variavel \"{}\" já declarada", var.id)
                }
            }
        }
        self.sectenvs[sect_num].vars.push(var);
    }

    /// Declara um novo global
    fn declare_global(&mut self, glb: parser::Global) {
        if self.glbs.len() > 0 {
            for v in &self.glbs {
                if v.identifier == glb.identifier {
                    abort!("Global \"{}\" já declarado!", glb.identifier)
                }
            }
        }
        self.glbs.push(glb);
    }

    /// Interpreta uma unidade sem executá-la
    pub fn interpret(&mut self, file: parser::Unit) {
        for global in file.globals {
            self.declare_global(global);
        }
        for sect in file.sects {
            self.sections.push(sect);
        }
        for cmd in file.glb_cmds {
            self.glb_cmds.push(cmd);
        }
    }

    /// Pega uma variavel do ambiente
    pub fn get_var(&self, name: &str) -> value::Value {
        // self.sectenvs é garantido a ter ao menos um elemento
        let mut ret = value::Value::Number(0.0);
        let mut found = false;
        for var in &self.sectenvs[self.sectenvs.len() - 1].vars {
            if var.id == name {
                ret = var.value.clone();
                found = true;
                break;
            }
        }
        if !found {
            // Se não encontrado, tente procurar nos globais.birl
            for glb in &self.glbs {
                if glb.identifier == name {
                    ret = value::parse_expr(&glb.value, self);
                    found = true;
                    break;
                }
            }
            if !found {
                // Ainda assim não achou
                abort!("Variável ou global não encontrado(a)!: \"{}\"", name);
            }
        }
        ret
    }

    /// Modifica o valor de uma variavel
    pub fn mod_var(&mut self, var: &str, newval: value::Value) {
        // Essa função deve procurar na pilha de variaveis da seção e nos globais.birl do programa
        let (mut index, mut found) = (0, false);
        loop {
            if index >= self.last_sectenv().vars.len() {
                break;
            }
            let ref mut v = self.last_sectenv().vars[index];
            if v.id == var {
                v.value = newval.clone();
                found = true;
                break;
            }
            index += 1;
        }
        if !found {
            if self.glbs.len() > 0 {
                // Não encontrado, procure nos globais.birl
                let mut index = 0;
                loop {
                    if index >= self.glbs.len() {
                        break;
                    }
                    let ref mut g = self.glbs[index];
                    if g.identifier == var {
                        if !g.is_const {
                            g.value = newval.as_str();
                            found = true;
                            break;
                        } else {
                            abort!("Tentativa de alterar o valor de global constante!")
                        }
                    }
                    index += 1;
                }
                if !found {
                    abort!("Variavel ou global não encontrado(a): \"{}\"", var)
                }
            } else {
                abort!("Variavel ou global não encontrado(a): \"{}\"", var)
            }
        }
    }

    // Inicio da implementação dos comandos

    /// Seta o valor de uma variável
    fn command_move(&mut self, target: String, val: value::Value) -> Option<CommandSignal> {
        self.mod_var(&target, val);
        None
    }

    /// Limpa o valor de uma variavel
    fn command_clear(&mut self, target: String) -> Option<CommandSignal> {
        self.mod_var(&target, value::Value::Number(0.0));
        None
    }

    /// Declara uma variavel com o valor padrão
    fn command_decl(&mut self, name: String) -> Option<CommandSignal> {
        let var = Variable::from(name, value::Value::Number(0.0));
        self.declare_var(var);
        None
    }

    /// Declara uma variavel com um valor padrão
    fn command_declwv(&mut self, name: String, val: value::Value) -> Option<CommandSignal> {
        let var = Variable::from(name, val);
        self.declare_var(var);
        None
    }

    /// Passa a execução para outra seção
    fn command_jump(&mut self, section: String) -> Option<CommandSignal> {
        let args = if section.contains('(') {
            // Expande os simbolos para uma lista de argumentos
            value::expand_sym_list(&section, self)
        } else {
            vec![]
        };
        let section = if section.contains('(') {
            let fpar = section.find('(').unwrap();
            section[..fpar].trim().to_string()
        } else {
            section
        };
        self.execute_section(&section, args);
        None
    }

    /// Compara dois valores
    fn command_cmp(&mut self, val1: value::Value, val2: value::Value) -> Option<CommandSignal> {
        self.last_cmp = compare(val1, val2);
        None
    }

    /// Executa uma seção caso comparação de equals
    fn command_cmp_eq(&mut self, sect: String) -> Option<CommandSignal> {
        if let Comparision::Equals = self.last_cmp {
            self.command_jump(sect);
        }
        None
    }

    /// Executa uma seção caso comparação dê not equals
    fn command_cmp_neq(&mut self, sect: String) -> Option<CommandSignal> {
        if let Comparision::More = self.last_cmp {
            self.command_jump(sect);
        } else if let Comparision::None = self.last_cmp {
            self.command_jump(sect);
        } else if let Comparision::Less = self.last_cmp {
            self.command_jump(sect);
        }
        None
    }

    /// Menos/Menor
    fn command_cmp_less(&mut self, sect: String) -> Option<CommandSignal> {
        if let Comparision::Less = self.last_cmp {
            self.command_jump(sect);
        }
        None
    }

    /// Menos/Menor ou igual
    fn command_cmp_lesseq(&mut self, sect: String) -> Option<CommandSignal> {
        if let Comparision::Less = self.last_cmp {
            self.command_jump(sect);
        } else if let Comparision::Equals = self.last_cmp {
            self.command_jump(sect);
        }
        None
    }

    /// Maior
    fn command_cmp_more(&mut self, sect: String) -> Option<CommandSignal> {
        if let Comparision::More = self.last_cmp {
            self.command_jump(sect);
        }
        None
    }

    /// Maior ou igual
    fn command_cmp_moreeq(&mut self, sect: String) -> Option<CommandSignal> {
        if let Comparision::More = self.last_cmp {
            self.command_jump(sect);
        } else if let Comparision::Equals = self.last_cmp {
            self.command_jump(sect);
        }
        None
    }

    /// Implementação do Print
    fn command_print(&mut self, messages: Vec<String>) -> Option<CommandSignal> {
        for message in messages {
            print!("{}", value::parse_expr(&message, self));
        }
        None
    }

    /// Implementação do Println
    fn command_println(&mut self, messages: Vec<String>) -> Option<CommandSignal> {
        if messages.len() > 0 {
            for msg in messages {
                print!("{}", value::parse_expr(&msg, self));
            }
        }
        println!("");
        None
    }

    /// Quit
    fn command_quit(&mut self, exit_code: String) -> Option<CommandSignal> {
        let exit_code = value::parse_expr(&exit_code, self);
        let exit_code = if let value::Value::Number(n) = exit_code {
            n as i32
        } else {
            0
        };
        Some(CommandSignal::Quit(exit_code))
    }

    fn command_return(&mut self, value: Option<String>) -> Option<CommandSignal> {
        // Coloca o valor de retorno na ultima seção, se for Some
        match value {
            Some(v) => {
                let num_sectenvs = self.sectenvs.len();
                if num_sectenvs < 2 {
                    abort!("Uso do return em ambiente global (proibido)")
                }
                let mut retvalue = Variable::from(String::from(parser::kw::KW_RETVAL_VAR), value::parse_expr(&v, self));
                retvalue.is_const = true; // O valor de retorno nao pode ser modificado
                self.declare_var_in(num_sectenvs - 2, retvalue); // num_sectenvs - 1 é a seção atual
            }
            None => {} // Nao modifica o valor de retorno se nenhum valor foi retornado
        }
        Some(CommandSignal::Return)
    }

    /// Input
    fn command_input(&mut self, var: String) -> Option<CommandSignal> {
        let input = get_input();
        self.mod_var(&var, value::Value::Str(Box::new(input)));
        None
    }

    /// Input upper
    fn command_input_upper(&mut self, var: String) -> Option<CommandSignal> {
        let input = get_input().to_uppercase();
        self.mod_var(&var, value::Value::Str(Box::new(input)));
        None
    }

    /// Executa um comando
    fn execute_command(&mut self, cmd: parser::Command) -> Option<CommandSignal> {
        use parser::Command;
        match cmd {
            Command::Move(trg, val) => {
                let val = value::parse_expr(&val, self);
                self.command_move(trg, val)
            }
            Command::Clear(trg) => self.command_clear(trg),
            Command::Decl(trg) => self.command_decl(trg),
            Command::DeclWV(trg, val) => {
                let val = value::parse_expr(&val, self);
                self.command_declwv(trg, val)
            }
            Command::Jump(sect) => self.command_jump(sect),
            Command::Cmp(val1, val2) => {
                let val1 = value::parse_expr(&val1, self);
                let val2 = value::parse_expr(&val2, self);
                self.command_cmp(val1, val2)
            }
            Command::CmpEq(sect) => self.command_cmp_eq(sect),
            Command::CmpNEq(sect) => self.command_cmp_neq(sect),
            Command::CmpLess(s) => self.command_cmp_less(s),
            Command::CmpLessEq(s) => self.command_cmp_lesseq(s),
            Command::CmpMore(s) => self.command_cmp_more(s),
            Command::CmpMoreEq(s) => self.command_cmp_moreeq(s),
            Command::Print(msg) => self.command_print(msg),
            Command::Println(msg) => self.command_println(msg),
            Command::Quit(exit_code) => self.command_quit(exit_code),
            Command::Return(val) => {
                self.command_return(val)
            }
            Command::Input(var) => self.command_input(var),
            Command::InputUpper(var) => self.command_input_upper(var),
        }
    }

    /// Executa uma seção, se preciso, recursivamente
    fn execute_section(&mut self, sect_name: &str, arguments: Vec<value::Value>) {
        use std::{process, thread, time};
        let mut section = parser::Section::new();
        let mut expected_args: Vec<parser::ExpectedParameter> = vec![];
        let mut found = false;
        for sect in &self.sections {
            if sect.name == sect_name {
                section = sect.clone();
                expected_args = sect.param_list.clone();
                found = true;
                break;
            }
        }
        if !found {
            abort!("Seção não encontrada: \"{}\".", sect_name)
        } else {
            let partial = section.partial;
            let recv_args = value::ValueType::types_of(&arguments);
            // A função em si já joga o erro caso o numero de parametros nao coincida
            if !received_params_match(&sect_name, &expected_args, &recv_args) {
                // Caso o tipo dos parametros passados seja diferente, de o erro dizendo tal
                abort!("Tipos dos parametros passados para \"{}\" não coincidem com os tipos \
                        declarados.",
                       sect_name)
            }
            // A criação de um novo ambiente só deve ser feita caso a seção não seja parcial
            if !partial {
                // Cria um novo ambiente pra nova seção no fim da pilha
                self.sectenvs.push(SectionEnvironment::new());
                // Cria a variavel que guarda o nome da seção
                self.last_sectenv()
                    .vars
                    .push(Variable::from(String::from("JAULA"),
                                         value::Value::Str(Box::new(String::from(sect_name)))));
                // Empurra os argumentos recebidos pra pilha de variaveis
                for i in 0..arguments.len() {
                    let var = Variable::from(expected_args[i].id.clone(), arguments[i].clone());
                    self.last_sectenv().vars.push(var);
                }
            }
            for cmd in section.lines {
                // Verifica se, caso essa seção não seja parcial, se a antiga seção deixou um sinal de return
                match self.last_sectenv().last_sig {
                    Some(ref s) => {
                        if let &CommandSignal::Return = s {
                            break; // Quebra o loop
                        }
                    }
                    None => {}
                }
                let sig = self.execute_command(cmd); // Pega o sinal retornado pelo comando
                match sig {
                    Some(s) => {
                        //self.last_sectenv().last_sig = Some(s.clone());
                        match s {
                            CommandSignal::Return => break, // Encerra a execução da seção atual
                            CommandSignal::Quit(code) => process::exit(code),
                            CommandSignal::Wait(secs) => thread::sleep(time::Duration::from_secs(secs)),
                        }
                    }
                    None => {
                        self.last_sectenv().last_sig = None; // Simplesmente altera na seção
                    }
                }
            }
            if !partial {
                self.sectenvs.pop(); // Joga fora a ultima seção, que é a atual, caso não seja parcial
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
        let var_cumpade: String = match env::var(user_varenv) {
            Ok(usr) => usr,
            Err(_) => var_names[0].to_string(), // CUMPADE
        };
        let var_values = vec![value::Value::Str(Box::new(var_cumpade.to_uppercase())),
                              value::Value::Number(1.0),
                              value::Value::Str(Box::new(String::from("BAMBAM")))];
        for i in 0..var_names.len() {
            let (name, val) = (var_names[i], var_values[i].clone());
            let glb = parser::Global {
                identifier: String::from(name),
                value: val.as_str(),
                is_const: true,
            };
            self.declare_global(glb);
        }
    }

    /// Verifica se nos arquivos interpretados há menção do main
    fn has_main(&self, main_sect: &str) -> bool {
        if self.sections.len() < 1 {
            false
        } else {
            let mut res = false;
            for sect in &self.sections {
                if sect.name == main_sect {
                    res = true;
                    break;
                }
            }
            res
        }
    }

    /// Executa a seção padrão e retorna o codigo de saida
    pub fn start_program(&mut self)  {
        self.init_variables();
        // Executa os comandos globais.birl
        if self.glb_cmds.len() > 0 {
            for i in 0..self.glb_cmds.len() {
                let cmd = self.glb_cmds[i].clone();
                self.execute_command(cmd);
            }
        }
        // Verifica se existe a função principal
        let has_main = self.has_main(&self.entry);
        if has_main {
            let entryp = self.entry.clone();
            self.execute_section(&entryp, vec![]);
        }
    }
}
