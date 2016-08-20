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
    /// Printa com nova linha
    pub const KW_PRINTLN: &'static str = "CE QUER VER ESSA PORRA";
    /// Printa
    pub const KW_PRINT: &'static str = "CE QUER VER";
    /// Sai do programa
    pub const KW_QUIT: &'static str = "BIRL";
}

#[cfg(target_pointer_width = "32")]
mod types {
    pub type MaxInt = i32;
    pub type MaxFlt = f32;
}

#[cfg(target_pointer_width = "64")]
mod types {
    pub type MaxInt = i64;
    pub type MaxFlt = f64;
}

/// Representa um valor que pode ser atribuido a uma variavel
#[derive(Clone)]
pub enum Value {
    /// Numero inteiro de 64 bits
    Integer(types::MaxInt),
    /// Numero de ponto flutuante de 64 bits
    FloatP(types::MaxFlt),
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
                    let mut sym = String::new();
                    for c in e.chars() {
                        match c {
                            // Checa por um caractere invalido
                            '+' | '-' | '\\' | '*' | '|' | '(' | ')' | '#' | ';' | '!' => break,
                            _ => sym.push(c),
                        }
                    }
                    return Some(Value::Symbol(sym));
                }
                _ => println!("Erro, caractere \"{}\" encontrado durante parsing.", fchar),
            }
            None
        }
    }
}

#[derive(Clone)]
/// Representa um comando, que é executado dentro do contexto atual
/// Os valores passados aos comandos têm nomes fantasia alfabéticos para exemplificação
pub enum Command {
    /// Move (copia) o conteudo da variavel no endereco a pro b
    Move(Value, Value),
    /// Limpa o valor da variavel no endereco a
    Clear(Value),
    /// Aplica xor na variavel no endereco a com o valor b
    Xor(Value, Value),
    /// Aplica and na variavel no endereco a com o valor b
    And(Value, Value),
    /// Aplica or  na variavel no endereco a com o valor b
    Or(Value, Value),
    /// Adiciona b ao valor da variavel no endereco a
    Add(Value, Value),
    /// Remove b do valor da variavel no endereco a
    Rem(Value, Value),
    /// Divide o valor da variavel no endereco a com o valor b
    Div(Value, Value),
    /// Multiplica o valor da variavel no endereco a com o valor b
    Mul(Value, Value),
    /// Multiplica um valor numa variavel por -1
    Neg(Value),
    /// Declara a variavel com nome a
    Decl(Value),
    /// Declara a variavel com nome a e valor b
    DeclWV(Value, Value),
    /// Passa a execução para outra seção com nome a, retornando uma instrução à frente
    Jump(Value),
    /// Compara os valores de a e b, usado em condicionais
    Cmp(Value, Value),
    /// Printa o valor a com uma nova linha em seguida
    Println(Value),
    /// Printa o valor a
    Print(Value),
    /// Sai do programa
    Quit,
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
    Println,
    Print,
    Quit,
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
        CommandType::Println => (1, kw::KW_PRINTLN),
        CommandType::Print => (1, kw::KW_PRINT),
        CommandType::Quit => (0, kw::KW_QUIT),
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
    let cmd_parts = cmd.split(':').map(|x| x.trim()).collect::<Vec<&str>>();
    // argumentos
    let mut arguments: Vec<&str> = Vec::new();
    // Tipo/nome do comando
    let cmd_type = if cmd_parts.len() > 1 {
        if n_of_char(',', cmd_parts[1]) == 0 {
            if cmd_parts[1].trim() != "" {
                // Um argumento
                arguments.push(cmd_parts[1].trim());
            }
        } else {
            arguments = cmd_parts[1].split(',').map(|arg| arg.trim()).collect();
        }
        cmd_parts[0]
    } else {
        cmd.trim()
    };
    let cmd: Option<Command> = match cmd_type {
        kw::KW_MOVE => {
            check_n_params(CommandType::Move, arguments.len());
            let (addr1, addr2) = (value::parse_expr(arguments[0]).unwrap(),
                                  value::parse_expr(arguments[1]).unwrap());
            Some(Command::Move(addr1, addr2))
        }
        kw::KW_CLEAR => {
            check_n_params(CommandType::Clear, arguments.len());
            let addr = value::parse_expr(arguments[0]).unwrap();
            Some(Command::Clear(addr))
        }
        kw::KW_XOR => {
            check_n_params(CommandType::Xor, arguments.len());
            let (addr1, addr2) = (value::parse_expr(arguments[0]).unwrap(),
                                  value::parse_expr(arguments[1]).unwrap());
            Some(Command::Xor(addr1, addr2))
        }
        kw::KW_AND => {
            check_n_params(CommandType::And, arguments.len());
            let (addr1, addr2) = (value::parse_expr(arguments[0]).unwrap(),
                                  value::parse_expr(arguments[1]).unwrap());
            Some(Command::And(addr1, addr2))
        }
        kw::KW_OR => {
            check_n_params(CommandType::Or, arguments.len());
            let (addr1, addr2) = (value::parse_expr(arguments[0]).unwrap(),
                                  value::parse_expr(arguments[1]).unwrap());
            Some(Command::Or(addr1, addr2))
        }
        kw::KW_ADD => {
            check_n_params(CommandType::Add, arguments.len());
            let (addr1, addr2) = (value::parse_expr(arguments[0]).unwrap(),
                                  value::parse_expr(arguments[1]).unwrap());
            Some(Command::Add(addr1, addr2))
        }
        kw::KW_REM => {
            check_n_params(CommandType::Rem, arguments.len());
            let (addr1, addr2) = (value::parse_expr(arguments[0]).unwrap(),
                                  value::parse_expr(arguments[1]).unwrap());
            Some(Command::Rem(addr1, addr2))
        }
        kw::KW_DIV => {
            check_n_params(CommandType::Div, arguments.len());
            let (addr1, addr2) = (value::parse_expr(arguments[0]).unwrap(),
                                  value::parse_expr(arguments[1]).unwrap());
            Some(Command::Div(addr1, addr2))
        }
        kw::KW_MUL => {
            check_n_params(CommandType::Mul, arguments.len());
            let (addr1, addr2) = (value::parse_expr(arguments[0]).unwrap(),
                                  value::parse_expr(arguments[1]).unwrap());
            Some(Command::Mul(addr1, addr2))
        }
        kw::KW_NEG => {
            check_n_params(CommandType::Neg, arguments.len());
            let addr = value::parse_expr(arguments[0]).unwrap();
            Some(Command::Neg(addr))
        }
        kw::KW_DECL => {
            check_n_params(CommandType::Decl, arguments.len());
            let name = value::parse_expr(arguments[0]).unwrap();
            Some(Command::Decl(name))
        }
        kw::KW_DECLWV => {
            check_n_params(CommandType::DeclWV, arguments.len());
            let (name, val) = (value::parse_expr(arguments[0]).unwrap(),
                               value::parse_expr(arguments[1]).unwrap());
            Some(Command::DeclWV(name, val))
        }
        kw::KW_JUMP => {
            check_n_params(CommandType::Jump, arguments.len());
            let section = value::parse_expr(arguments[0]).unwrap();
            Some(Command::Jump(section))
        }
        kw::KW_CMP => {
            check_n_params(CommandType::Cmp, arguments.len());
            let (addr1, addr2) = (value::parse_expr(arguments[0]).unwrap(),
                                  value::parse_expr(arguments[1]).unwrap());
            Some(Command::Cmp(addr1, addr2))
        }
        kw::KW_PRINTLN => {
            check_n_params(CommandType::Println, arguments.len());
            let val = value::parse_expr(arguments[0]).unwrap();
            Some(Command::Println(val))
        }
        kw::KW_PRINT => {
            check_n_params(CommandType::Print, arguments.len());
            let val = value::parse_expr(arguments[0]).unwrap();
            Some(Command::Print(val))
        }
        kw::KW_QUIT => Some(Command::Quit),
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
    pub sects: Vec<Section>,
    /// Conjunto de globais
    pub consts: Vec<Global>,
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
    let mut parsing_section = false;
    let mut cur_section: Vec<String> = vec![];
    loop {
        let line: String = match lines.next() {
            Some(l) => {
                match l {
                    Ok(ll) => String::from(ll.trim()),
                    Err(_) => String::new(),
                }
            }
            None => break,
        };
        // Divide a string em palavras separadas por um espaço
        let words = line.split(' ').collect::<Vec<&str>>();
        // Verifica a primeira palavra da linha
        match words[0].trim() {
            // Se for declaração de um global, empurra o global pra unit
            kw::KW_GLOBAL => final_unit.consts.push(parse_global(&line)),
            // Se for declaração de uma seção, começa o parsing da seção
            kw::KW_SECTION => {
                cur_section.push(line.clone());
                parsing_section = true;
            }
            // FIXME: Hardcode da palavra, mude depois pra uma forma de verificar dinamicamente
            "SAINDO" if parsing_section => {
                cur_section.push(line.clone());
                if line.trim() == kw::KW_SECTEND {
                    // Encerra seção
                    parsing_section = false;
                    final_unit.sects.push(parse_section(cur_section.clone()));
                    cur_section.clear();
                }
            }
            // Quando estiver dentro de uma seção, empurre o comando pra seção
            _ if parsing_section => {
                cur_section.push(line.clone());
            }
            // Se não for nenhuma (os comandos só são interpretados dentro da seção)
            _ => {
                panic!("Erro: \"{}\" não entendida no contexto global. Seção: {}",
                       line,
                       cur_section[0])
            }
        }
    }
    final_unit
}

#[derive(Clone)]
/// Representa uma área chamável que pode ser executada
pub struct Section {
    /// Nome da seção
    pub name: String,
    /// Conjunto de linhas/comandos para execução
    pub lines: Vec<Command>,
}

impl Section {
    pub fn new() -> Section {
        Section {
            name: String::new(),
            lines: vec![],
        }
    }
}

/// Faz parsing de uma seção
fn parse_section(lines: Vec<String>) -> Section {
    // Separa a seção em linha
    if lines.len() <= 1 {
        panic!("Erro fazendo parsing da seção. Número incorreto de linhas: {}.",
               lines.len())
    } else {
        // Checagens de declaração e finalização são feitas em parse
        // Declaração de uma seção:
        // PALAVRA_CHAVE nome
        if !lines[0].contains(' ') {
            panic!("Erro na declaração da seção! Falta nome depois da palavra chave");
        }
        let mut sect = Section {
            name: String::from(lines[0].split(' ').collect::<Vec<&str>>()[1].trim()),
            lines: vec![],
        };
        // O -1 é pra não contar com a ultima linha, o SAINDO DA JAULA
        for l in 1..lines.len() - 1 {
            let ref line = lines[l];
            if line.len() <= 0 {
                continue;
            }
            // Se a linha não tem nada de util até um comentario, pula ela
            if line.chars().collect::<Vec<char>>()[0] == '#' ||
               line.chars().collect::<Vec<char>>()[0] == ';' {
                continue;
            }
            // Parte util da linha, caso haja um comentario
            // Comentarios são feitos com # ou ;
            let util_line = if line.contains('#') || line.contains(';') {
                let mut tmp = String::new();
                for c in line.chars() {
                    // Enquanto o caractere não for um comentário, continue
                    if c != '#' && c != ';' {
                        tmp.push(c);
                    } else {
                        break;
                    }
                }
                tmp
            } else {
                line.clone()
            };
            sect.lines.push(parse_cmd(&util_line).unwrap());
        }
        sect
    }
}

/// Representa um valor global, constante
pub struct Global {
    /// Identificador do valor global
    pub identifier: String,
    /// Valor do global
    pub value: Value,
}

/// Faz parsing de um global
fn parse_global(glb: &str) -> Global {
    // Estrutura da declaração de um global: PALAVRA_CHAVE: nome: valor
    let global = String::from(glb.trim());
    let words = global.split(':').map(|x| x.trim()).collect::<Vec<&str>>();
    // Separa o nome e valor do global
    let (glb_name, glb_value) = (words[1], words[2]);
    let glb_value = match value::parse_expr(glb_value) {
        Some(val) => val,
        None => {
            panic!("Erro fazendo parsing do global \"{}\", valor incorreto.",
                   glb_name)
        }
    };
    Global {
        identifier: String::from(glb_name),
        value: glb_value,
    }
}
