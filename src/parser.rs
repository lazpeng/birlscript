//! Responsavel pelo parsing e de gerar a AST do programa BIRL

#![allow(dead_code)]

/// Representa as keywords da linguagem
pub mod kw {
    // Definições
    /// Usada para declaração de globais
    pub const KW_GLOBAL: &'static str = "SAI DE CASA";
    /// Usada para definição de seções
    pub const KW_SECTION: &'static str = "JAULA";
    /// Usada para finalizar a definição de seções
    pub const KW_SECTEND: &'static str = "SAINDO DA JAULA";

    // Comandos
    /// Copia o valor de uma variavel a outra
    pub const KW_MOVE: &'static str = "BORA";
    /// Limpa o valor de uma variavel
    pub const KW_CLEAR: &'static str = "NUM VAI DA NAO";
    /// Xor (operador binário)
    pub const KW_XOR: &'static str = "TRAPEZIO DESCENDENTE";
    /// And (operador binário)
    pub const KW_AND: &'static str = "FIBRA";
    /// Or (operador binário)
    pub const KW_OR: &'static str = "TRAPEZIO";
    /// Adição
    pub const KW_ADD: &'static str = "CONSTROI";
    /// Diminuição
    pub const KW_REM: &'static str = "MENOS CUMPADE";
    /// Divisão
    pub const KW_DIV: &'static str = "DIVIDE CUMPADE";
    /// Multiplicação
    pub const KW_MUL: &'static str = "CONSTROI FIBRA";
    /// Multiplica por -1
    pub const KW_NEG: &'static str = "NEGATIVA BAMBAM";
    /// Declara uma variável
    pub const KW_DECL: &'static str = "VEM";
    /// Declara uma variável com um valor
    pub const KW_DECLWV: &'static str = "VEM PORRA";
    /// Realiza um "pulo" de uma seção para outra
    pub const KW_JUMP: &'static str = "E HORA DO";
    /// Comparação
    pub const KW_CMP: &'static str = "E ELE QUE A GENTE QUER";
}

/// Diferenciação dos tipos de diferentes arquiteturas
pub mod types {
    #[cfg(target_pointer_width = "32")]
    /// No caso de um pc de 32 bits, use uint32 como endereço
    pub type Address = u32;

    #[cfg(target_pointer_width = "32")]
    /// No caso de um pc de 32 bits, use uint32 como maior valor int signed
    pub type MaxSInt = i32;

    #[cfg(target_pointer_width = "32")]
    /// No caso de um pc de 32 bits, use float como maior valor de ponto flutuante
    pub type MaxFP = f32;

    #[cfg(target_pointer_width = "64")]
    /// No caso de um p de 64 bits, use uint64 como endereço
    pub type Address = u64;

    #[cfg(target_pointer_width = "64")]
    /// No caso de um pc de 32 bits, use uint64 como maior valor int signed
    pub type MaxSInt = i64;

    #[cfg(target_pointer_width = "64")]
    /// No caso de um pc de 32 bits, use double como maior valor de ponto flutuante
    pub type MaxFP = f64;
}

/// Representa um valor que pode ser atribuido a uma variavel
pub enum Value {
    /// Numero inteiro de 64 bits
    Integer(types::MaxSInt),
    /// Numero de ponto flutuante de 64 bits
    FloatP(types::MaxFP),
    /// Caractere UTF-8
    Char(char),
    /// Texto em UTF-8, guarda apenas a referencia ao valor no heap
    Str(Box<String>),
    /// Simbolo, que representa uma variavel no contexto do programa
    Symbol(String),
}

/// Sub-modulo responsavel por fazer parsing de expressões
mod value {

    use super::*;

    /// Tenta fazer parsing de uma expressão e retornar seu valor
    pub fn parse_expr(expr: &str) -> Option<Value> {
        let e = String::from(expr.trim());
        if e == "" {
            None
        } else {
            let fchar = e.chars().collect::<Vec<char>>()[0];
            match fchar {
                '0'...'9' => {}
                '\"' => {
                    // last_escape é se o ultimo caractere foi uma barra de escape, ignore ela
                    let (mut value, mut last_escape) = (String::new(), false);
                    let chars = e.chars().collect::<Vec<char>>();
                    for c in 1..chars.len() {
                        let actual = chars[c];
                        if actual == '\\' {
                            if last_escape {
                                value.push('\\');
                                last_escape = false;
                            } else {
                                last_escape = true;
                            }
                            continue;
                        }
                        if last_escape {
                            match actual {
                                'n' => value.push('\n'),
                                '\"' => value.push('\"'),
                                't' => value.push('\t'),
                                'r' => value.push('\r'),
                                '\'' => value.push('\''),
                                _ => {
                                    println!("Aviso: Sequencia de escape não reconhecida: \"{}\".",
                                             actual)
                                }
                            }
                            last_escape = false;
                            continue;
                        }
                        if actual == '\"' {
                            // Fim da string
                            return Some(Value::Str(Box::new(value)));
                        }
                        value.push(actual);
                    }
                }
                'a'...'z' | 'A'...'Z' | '_' => {
                    return Some(Value::Symbol(e));
                }
                _ => println!("Erro, caractere \"{}\" encontrado durante parsing.", fchar),
            }
            None
        }
    }
}

/// Representa um comando, que é executado dentro do contexto atual
/// Os valores passados aos comandos têm nomes fantasia alfabéticos para exemplificação
pub enum Command {
    /// Move (copia) o conteudo da variavel no endereco a pro b
    Move(types::Address, types::Address),
    /// Limpa o valor da variavel no endereco a
    Clear(types::Address),
    /// Aplica xor na variavel no endereco a com o valor b
    Xor(types::Address, Value),
    /// Aplica and na variavel no endereco a com o valor b
    And(types::Address, Value),
    /// Aplica or  na variavel no endereco a com o valor b
    Or(types::Address, Value),
    /// Adiciona b ao valor da variavel no endereco a
    Add(types::Address, Value),
    /// Remove b do valor da variavel no endereco a
    Rem(types::Address, types::MaxSInt),
    /// Divide o valor da variavel no endereco a com o valor b
    Div(types::Address, types::MaxSInt),
    /// Multiplica o valor da variavel no endereco a com o valor b
    Mul(types::Address, types::MaxSInt),
    /// Multiplica um valor numa variavel por -1
    Neg(types::Address),
    /// Declara a variavel com nome a
    Decl(String),
    /// Declara a variavel com nome a e valor b
    DeclWV(String, Value),
    /// Passa a execução para outra seção com nome a, retornando uma instrução à frente
    Jump(String),
    /// Compara os valores de a e b, usado em condicionais
    Cmp(Value, Value),
}

/// Facil representação dos comandos sem os argumentos
pub enum CommandType {
    Move,
    Clear,
    Xor,
    And,
    Or,
    Add,
    Rem,
    Div,
    Mul,
    Neg,
    Decl,
    DeclWV,
    Jump,
    Cmp,
}

/// Procura pelo caractere c em src e retorna quantas vezes ele foi encontrado
fn n_of_char(c: char, src: &str) -> i32 {
    if src.len() <= 0 {
        0
    } else {
        let mut num = 0i32;
        for curr in src.chars() {
            if curr == c {
                num += 1;
            }
        }
        num
    }
}

/// Verifica se foi passada a quantidade correta de argumentos para um comando
fn check_n_params(command: CommandType, num_params: usize) {
    // Pra cada comando, retorne um valor inteiro para o numero de parametros
    let (expected, id) = match command {
        CommandType::Cmp => (2, kw::KW_CMP),
        CommandType::Jump => (1, kw::KW_JUMP),
        CommandType::DeclWV => (2, kw::KW_DECLWV),
        CommandType::Decl => (1, kw::KW_DECL),
        CommandType::Neg => (1, kw::KW_NEG),
        CommandType::Mul => (2, kw::KW_MUL),
        CommandType::Div => (2, kw::KW_DIV),
        CommandType::Rem => (2, kw::KW_REM),
        CommandType::Add => (2, kw::KW_ADD),
        CommandType::Or => (2, kw::KW_OR),
        CommandType::And => (2, kw::KW_AND),
        CommandType::Xor => (2, kw::KW_XOR),
        CommandType::Clear => (1, kw::KW_CLEAR),
        CommandType::Move => (2, kw::KW_MOVE),
    };
    if expected != num_params {
        panic!(format!("Erro: \"{}\" espera {} parametros, porém {} foram passados.",
                       id,
                       expected,
                       num_params));
    }
}

/// Faz parsing de um comando
fn parse_cmd(cmd: &str) -> Option<Command> {
    // Estrutura de um comando:
    // COMANDO: var1, var2, ...
    let cmd = cmd.trim();
    let cmd_parts = cmd.split(':').collect::<Vec<&str>>();
    // Tipo/nome do comando
    let cmd_type = cmd_parts[0];
    let mut arguments: Vec<&str> = vec![];
    if n_of_char(',', cmd_parts[1]) == 0 {
        // Apenas um argumento
        arguments.push(cmd_parts[1].trim());
    } else {
        arguments = cmd_parts[1].split(',').map(|arg| arg.trim()).collect::<Vec<&str>>();
    }
    let cmd: Option<Command> = match cmd_type {
        kw::KW_MOVE => {
            check_n_params(CommandType::Move, arguments.len());
            let (addr1, addr2) = (arguments[0].parse::<types::Address>().unwrap(),
                                  arguments[1].parse::<types::Address>().unwrap());
            Some(Command::Move(addr1, addr2))
        }
        kw::KW_CLEAR => {
            check_n_params(CommandType::Clear, arguments.len());
            let addr = arguments[0].parse::<types::Address>().unwrap();
            Some(Command::Clear(addr))
        }
        kw::KW_XOR => {
            check_n_params(CommandType::Xor, arguments.len());
            let (addr1, addr2) = (arguments[0].parse::<types::Address>().unwrap(),
                                  value::parse_expr(arguments[1]).unwrap());
            Some(Command::Xor(addr1, addr2))
        }
        kw::KW_AND => {
            check_n_params(CommandType::And, arguments.len());
            let (addr1, addr2) = (arguments[0].parse::<types::Address>().unwrap(),
                                  value::parse_expr(arguments[1]).unwrap());
            Some(Command::And(addr1, addr2))
        }
        kw::KW_OR => {
            check_n_params(CommandType::Or, arguments.len());
            let (addr1, addr2) = (arguments[0].parse::<types::Address>().unwrap(),
                                  value::parse_expr(arguments[1]).unwrap());
            Some(Command::Or(addr1, addr2))
        }
        kw::KW_ADD => {
            check_n_params(CommandType::Add, arguments.len());
            let (addr1, addr2) = (arguments[0].parse::<types::Address>().unwrap(),
                                  value::parse_expr(arguments[1]).unwrap());
            Some(Command::Add(addr1, addr2))
        }
        kw::KW_REM => {
            check_n_params(CommandType::Rem, arguments.len());
            let (addr1, addr2) = (arguments[0].parse::<types::Address>().unwrap(),
                                  arguments[1].parse::<types::MaxSInt>().unwrap());
            Some(Command::Rem(addr1, addr2))
        }
        kw::KW_DIV => {
            check_n_params(CommandType::Div, arguments.len());
            let (addr1, addr2) = (arguments[0].parse::<types::Address>().unwrap(),
                                  arguments[1].parse::<types::MaxSInt>().unwrap());
            Some(Command::Div(addr1, addr2))
        }
        kw::KW_MUL => {
            check_n_params(CommandType::Mul, arguments.len());
            let (addr1, addr2) = (arguments[0].parse::<types::Address>().unwrap(),
                                  arguments[1].parse::<types::MaxSInt>().unwrap());
            Some(Command::Mul(addr1, addr2))
        }
        kw::KW_NEG => {
            check_n_params(CommandType::Neg, arguments.len());
            let addr = arguments[0].parse::<types::Address>().unwrap();
            Some(Command::Neg(addr))
        }
        kw::KW_DECL => {
            check_n_params(CommandType::Decl, arguments.len());
            let name = String::from(arguments[0]);
            Some(Command::Decl(name))
        }
        kw::KW_DECLWV => {
            check_n_params(CommandType::DeclWV, arguments.len());
            let (name, val) = (String::from(arguments[0]),
                               value::parse_expr(arguments[1]).unwrap());
            Some(Command::DeclWV(name, val))
        }
        kw::KW_JUMP => {
            check_n_params(CommandType::Jump, arguments.len());
            let section = String::from(arguments[0]);
            Some(Command::Jump(section))
        }
        kw::KW_CMP => {
            check_n_params(CommandType::Cmp, arguments.len());
            let (addr1, addr2) = (value::parse_expr(arguments[0]).unwrap(),
                                  value::parse_expr(arguments[1]).unwrap());
            Some(Command::Cmp(addr1, addr2))
        }
        _ => {
            println!("Erro: Comando \"{}\" não existe.", cmd_type);
            None
        }
    };
    cmd
}

/// Representa uma unidade (arquivo compilado) contendo o conteudo a ser executado
pub struct Unit {
    /// Conjunto de seções para execução
    sects: Vec<Section>,
    /// Conjunto de globais
    consts: Vec<Global>,
}

/// Realiza a interpretação de um arquivo e retorna sua unidade compilada
pub fn parse(file: &str) -> Unit {
    use std::fs;
    use std::io::{BufRead, BufReader};
    let f = match fs::File::open(file) {
        Ok(a) => a,
        Err(e) => panic!(e),
    };
    // Valor de retorno
    let mut final_unit = Unit {
        sects: vec![],
        consts: vec![],
    };
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    // Se está fazendo parsing de uma seção e o conteudo atual da seção
    let (mut parsing_section, mut cur_section) = (false, String::new());
    loop {
        let line = match lines.next() {
            Some(l) => {
                match l {
                    Ok(ll) => ll,
                    Err(_) => break,
                }
            }
            None => break,
        };
        if parsing_section {
            cur_section.push_str(&line);
            if line.trim() == kw::KW_SECTEND {
                // Encerra seção
                parsing_section = false;
                final_unit.sects.push(parse_section(&cur_section));
                cur_section.clear();
                continue;
            }
        }
        // Divide a string em palavras separadas por um espaço
        let words = line.split(' ').collect::<Vec<&str>>();
        // Verifica a primeira palavra da linha
        match words[0].trim() {
            // Se for declaração de um global, empurra o global pra unit
            kw::KW_GLOBAL => final_unit.consts.push(parse_global(words[0])),
            // Se for declaração de uma seção, começa o parsing da seção
            kw::KW_SECTION => {
                cur_section.push_str(words[0]);
                parsing_section = true;
            }
            // Se não for nenhuma (os comandos só são interpretados dentro da seção)
            _ => panic!("Erro: \"{}\" não entendida no contexto global."),
        }
    }
    final_unit
}

/// Representa uma área chamável que pode ser executada
pub struct Section {
    /// Nome da seção
    name: String,
    /// Conjunto de linhas/comandos para execução
    lines: Vec<Command>,
}

/// Faz parsing de uma seção
fn parse_section(sect_str: &str) -> Section {
    // Separa a seção em linhas
    let lines = sect_str.split('\n').collect::<Vec<&str>>();
    if lines.len() <= 1 {
        panic!("Erro fazendo parsing da seção. Número incorreto de linhas.")
    } else {
        // Checagens de declaração e finalização são feitas em parse
        // Declaração de uma seção:
        // PALAVRA_CHAVE nome
        if !lines[0].contains(' ') {
            panic!("Erro na declaração da seção! Falta nome depois da palavra chave");
        }
        let mut sect = Section {
            name: String::from(lines[0].split(' ').collect::<Vec<&str>>()[0]),
            lines: vec![],
        };
        for l in 1..lines.len() - 1 {
            let line = lines[l].trim();
            // Se a linha não tem nada de util até um comentario, pula ela
            if line.chars().collect::<Vec<char>>()[0] == '#' {
                continue;
            }
            // Parte util da linha, caso haja um comentario
            let util_line = if line.contains('#') {
                let mut tmp = String::new();
                for c in line.chars() {
                    // Enquanto o caractere não for um comentário, continue
                    if c != '#' {
                        tmp.push(c);
                    } else {
                        break;
                    }
                }
                tmp
            } else {
                String::from(line)
            };
            sect.lines.push(parse_cmd(&util_line).unwrap());
        }
        sect
    }
}

/// Representa um valor global, constante
pub struct Global {
    /// Identificador do valor global
    identifier: String,
    /// Valor do global
    value: Value,
}

/// Faz parsing de um global
fn parse_global(glb: &str) -> Global {
    // Estrutura da declaração de um global: PALAVRA_CHAVE: nome: valor
    let global = String::from(glb.trim());
    let words = global.split(':').collect::<Vec<&str>>();
    // Separa o nome e valor do global
    let (glb_name, glb_value) = (words[1].trim(), words[2].trim());
    let glb_value = match value::parse_expr(glb_value) {
        Some(val) => val,
        None => panic!(),
    };
    Global {
        identifier: String::from(glb_name),
        value: glb_value,
    }
}

/// Modulo de teste
mod tests {

    #[test]
    fn command_decl() {
        let cmd = super::parse_cmd("VEM: MONSTRO").unwrap();
        match cmd {
            super::Command::Decl(name) => assert!(name == "MONSTRO"),
            _ => panic!(),
        }
    }

    #[test]
    fn command_declwv() {
        let cmd = super::parse_cmd("VEM PORRA: MONSTRO, \"CUMPADE\"").unwrap();
        match cmd {
            super::Command::DeclWV(name, val) => {
                println!("nome: {}", name);
                assert!(name == "MONSTRO");
                match val {
                    super::Value::Str(v) => {
                        println!("val: {}", v);
                        assert!(*v == "CUMPADE");
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
}
