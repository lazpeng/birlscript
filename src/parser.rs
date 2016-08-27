//! Responsavel pelo parsing e de gerar a AST do programa BIRL

use error;

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
    /// Declara uma variável
    pub const KW_DECL: &'static str = "VEM";
    /// Declara uma variável com um valor
    pub const KW_DECLWV: &'static str = "VEM PORRA";
    /// Realiza um "pulo" de uma seção para outra
    pub const KW_JUMP: &'static str = "E HORA DO";
    /// Comparação
    pub const KW_CMP: &'static str = "E ELE QUE A GENTE QUER";
    /// Comparação resultou em igual
    pub const KW_CMP_EQ: &'static str = "E ELE MEMO";
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

#[derive(Clone)]
/// Representa um comando, que é executado dentro do contexto atual
/// Os valores passados aos comandos têm nomes fantasia alfabéticos para exemplificação
pub enum Command {
    /// Move (copia) o conteudo da variavel no endereco a pro b
    Move(String, String),
    /// Limpa o valor da variavel no endereco a
    Clear(String),
    /// Declara a variavel com nome a
    Decl(String),
    /// Declara a variavel com nome a e valor b
    DeclWV(String, String),
    /// Passa a execução para outra seção com nome a, retornando uma instrução à frente
    Jump(String),
    /// Compara os valores de a e b, usado em condicionais
    Cmp(String, String),
    /// Executa seção a caso ultima comparação seja igual
    CmpEq(String),
    /// Printa o valor a com uma nova linha em seguida
    Println(String),
    /// Printa o valor a
    Print(String),
    /// Sai do programa
    Quit,
}

// Por algum motivo, o compilador acusa Quit como dead_code :/
#[allow(dead_code)]
/// Facil representação dos comandos sem os argumentos
pub enum CommandType {
    Move,
    Clear,
    Decl,
    DeclWV,
    Jump,
    Cmp,
    CmpEq,
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
        let mut last_char = ' ';
        let mut is_string = false;
        for cur in src.chars() {
            if cur == '\"' || cur == '\'' {
                // String ou caractere, verifica o ultimo caractere
                if last_char == '\\' {
                    // caractere de escape, ignora
                } else {
                    // inicia ou finaliza string ou char
                    is_string = !is_string;
                }
            }
            if !is_string {
                if cur == c {
                    num += 1;
                }
            }
            last_char = cur;
        }
        num
    }
}

/// Troca caracteres acentuados para suas versões sem acento
fn change_accents(src: &str) -> String {
    let mut nstr = String::new();
    for c in src.chars() {
        nstr.push(match c {
            'Á' | 'Ã'| 'À' => 'A',
            'É' => 'E',
            'Õ' | 'Ô' => 'O',
            'Í' => 'I',
            _ => c,
        });
    }
    nstr
}

/// Verifica se foi passada a quantidade correta de argumentos para um comando
fn check_n_params(command: CommandType, num_params: usize) {
    // Pra cada comando, retorne um valor inteiro para o numero de parametros
    let (expected, id) = match command {
        CommandType::Cmp => (2, kw::KW_CMP),
        CommandType::CmpEq => (1, kw::KW_CMP_EQ),
        CommandType::Jump => (1, kw::KW_JUMP),
        CommandType::DeclWV => (2, kw::KW_DECLWV),
        CommandType::Decl => (1, kw::KW_DECL),
        CommandType::Clear => (1, kw::KW_CLEAR),
        CommandType::Move => (2, kw::KW_MOVE),
        CommandType::Println => (1, kw::KW_PRINTLN),
        CommandType::Print => (1, kw::KW_PRINT),
        CommandType::Quit => (0, kw::KW_QUIT)
    };
    if expected != num_params {
        error::abort(&format!("\"{}\" espera {} parametros, porém {} foram passados.",
                       id,
                       expected,
                       num_params));
    }
}

/// Faz parsing de um comando
fn parse_cmd(cmd: &str) -> Command {
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
    let cmd: Command = match cmd_type {
        kw::KW_MOVE => {
            check_n_params(CommandType::Move, arguments.len());
            let (addr1, addr2) = (String::from(arguments[0]), String::from(arguments[1]));
            Command::Move(addr1, addr2)
        }
        kw::KW_CLEAR => {
            check_n_params(CommandType::Clear, arguments.len());
            Command::Clear(String::from(arguments[0]))
        }
        kw::KW_DECL => {
            check_n_params(CommandType::Decl, arguments.len());
            Command::Decl(String::from(arguments[0]))
        }
        kw::KW_DECLWV => {
            check_n_params(CommandType::DeclWV, arguments.len());
            let (name, val) = (String::from(arguments[0]), String::from(arguments[1]));
            Command::DeclWV(name, val)
        }
        kw::KW_JUMP => {
            check_n_params(CommandType::Jump, arguments.len());
            Command::Jump(String::from(arguments[0]))
        }
        kw::KW_CMP => {
            check_n_params(CommandType::Cmp, arguments.len());
            let (addr1, addr2) = (String::from(arguments[0]), String::from(arguments[1]));
            Command::Cmp(addr1, addr2)
        }
        kw::KW_CMP_EQ => {
            check_n_params(CommandType::CmpEq, arguments.len());
            Command::CmpEq(arguments[0].to_string())
        }
        kw::KW_PRINTLN => {
            check_n_params(CommandType::Println, arguments.len());
            Command::Println(String::from(arguments[0]))
        }
        kw::KW_PRINT => {
            check_n_params(CommandType::Print, arguments.len());
            Command::Print(String::from(arguments[0]))
        }
        kw::KW_QUIT => Command::Quit,
        _ => {
            error::abort(&format!("Comando \"{}\" não existe.", cmd_type));
            unreachable!()
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
        Err(e) => panic!("Erro abrindo arquivo \"{}\": \"{}\"", file, e),
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
        if line.trim() == "" {
            continue;
        }
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
                // Mude os acentos para que aceite comandos com acento
                cur_section.push(change_accents(&line));
            }
            // Se não for nenhuma (os comandos só são interpretados dentro da seção)
            _ => {
                error::abort(&format!("\"{}\" não entendida no contexto global. Seção: {}",
                       line,
                       cur_section[0]))
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
        error::abort(&format!("Erro fazendo parsing da seção. Número incorreto de linhas: {}.",
               lines.len()));
        unreachable!() // O abort sai do programa, logo esse codigo nunca sera executado
    } else {
        // Checagens de declaração e finalização são feitas em parse
        // Declaração de uma seção:
        // PALAVRA_CHAVE nome
        if !lines[0].contains(' ') {
            error::abort("Erro na declaração da seção! Falta nome depois da palavra chave");
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
            sect.lines.push(parse_cmd(&util_line));
        }
        sect
    }
}

/// Representa um valor global, constante
pub struct Global {
    /// Identificador do valor global
    pub identifier: String,
    /// Valor do global
    pub value: String,
}

/// Faz parsing de um global
fn parse_global(glb: &str) -> Global {
    // Estrutura da declaração de um global: PALAVRA_CHAVE: nome: valor
    let global = String::from(glb.trim());
    let words = global.split(':').map(|x| x.trim()).collect::<Vec<&str>>();
    // Separa o nome e valor do global
    let (glb_name, glb_value) = (words[1], String::from(words[2]));
    Global {
        identifier: String::from(glb_name),
        value: glb_value,
    }
}
