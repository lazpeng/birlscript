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
    pub const KW_DECLWV: &'static str = "VEM, PORRA";
    /// Realiza um "pulo" de uma seção para outra
    pub const KW_JUMP: &'static str = "E HORA DO";
    /// Comparação
    pub const KW_CMP: &'static str = "E ELE QUE A GENTE QUER";
    /// Comparação resultou em igual
    pub const KW_CMP_EQ: &'static str = "E ELE MEMO";
    /// Comparação resultou em diferente
    pub const KW_CMP_NEQ: &'static str = "NUM E ELE";
    /// Comparação resultou em menor
    pub const KW_CMP_LESS: &'static str = "MENOR, CUMPADE";
    /// Comparação resultou em menor ou igual
    pub const KW_CMP_LESSEQ: &'static str = "MENOR OU E MEMO";
    /// Comparação resultou em maior
    pub const KW_CMP_MORE: &'static str = "MAIOR, CUMPADE";
    /// Comparação resultou em maior ou igual
    pub const KW_CMP_MOREEQ: &'static str = "MAIOR OU E MEMO";
    /// Printa com nova linha
    pub const KW_PRINTLN: &'static str = "CE QUER VER ESSA PORRA";
    /// Printa
    pub const KW_PRINT: &'static str = "CE QUER VER";
    /// Sai do programa
    pub const KW_QUIT: &'static str = "BIRL";
    /// Pega uma string da entrada padrão
    pub const KW_INPUT: &'static str = "BORA CUMPADE";
    /// Pega uma string da entrada padrão com letras maiusculas
    pub const KW_INPUT_UP: &'static str = "BORA CUMPADE, PORRA";
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
    /// Executa seção caso a ultima comparação seja diferente
    CmpNEq(String),
    /// Executa a seção caso a ultima comparação seja menor
    CmpLess(String),
    /// Executa a seção caso a ultima comparação seja menor ou igual
    CmpLessEq(String),
    /// Executa a seção caso a ultima comparação seja maior
    CmpMore(String),
    /// Executa a seção caso a ultima comparação seja maior ou igual
    CmpMoreEq(String),
    /// Printa uma série de valores com uma nova linha em seguida
    Println(Vec<String>),
    /// Printa uma série de valores
    Print(Vec<String>),
    /// Sai do programa
    Quit,
    /// Le a entrada padrão pra uma variavel
    Input(String),
    /// Le a entrada padrão e retorna um uppercase
    InputUpper(String),
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
    CmpNEq,
    CmpLess,
    CmpLessEq,
    CmpMore,
    CmpMoreEq,
    Println,
    Print,
    Quit,
    Input,
    InputUpper,
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
        CommandType::CmpNEq => (1, kw::KW_CMP_NEQ),
        CommandType::CmpLess => (1, kw::KW_CMP_LESS),
        CommandType::CmpLessEq => (1, kw::KW_CMP_LESSEQ),
        CommandType::CmpMore => (1, kw::KW_CMP_MORE),
        CommandType::CmpMoreEq => (1, kw::KW_CMP_MOREEQ),
        CommandType::Jump => (1, kw::KW_JUMP),
        CommandType::DeclWV => (2, kw::KW_DECLWV),
        CommandType::Decl => (1, kw::KW_DECL),
        CommandType::Clear => (1, kw::KW_CLEAR),
        CommandType::Move => (2, kw::KW_MOVE),
        // No caso do print e println, eles aceitam mais de um argumento, então faça uma checagem adicional
        CommandType::Println => {
            // No caso do println, ele pode ser usado sem um argumento, assim printando apenas uma nova linha
            (num_params, kw::KW_PRINTLN)
        }
        CommandType::Print => {
            // Print não
            if num_params < 1 {
                (1, kw::KW_PRINT)
            } else {
                (num_params, kw::KW_PRINTLN)
            }   
        }
        CommandType::Quit => (0, kw::KW_QUIT),
        CommandType::Input => (1, kw::KW_INPUT),
        CommandType::InputUpper => (1, kw::KW_INPUT_UP),
    };
    if expected != num_params {
        error::abort(&format!("\"{}\" espera {} parametros, porém {} foram passados.",
                       id,
                       expected,
                       num_params));
    }
}

fn split_arguments(args: String) -> Vec<String> {
    if args == "" {
        vec![]
    } else {
        let mut result: Vec<String> = vec![];
        let (mut in_str, mut in_char, mut last_escape) = (false, false, false);
        let mut last_arg = String::new();
        for c in args.chars() {
            match c {
                '\"' if in_str => {
                    if last_escape {
                        last_escape = false;
                        last_arg.push_str("\\\"");
                    } else {
                        in_str = false;
                        last_arg.push('\"');
                    }
                }
                '\"' => {
                    in_str = true;
                    last_arg.push('\"');
                }
                '\'' => {
                    last_arg.push(c);
                    if !in_str {
                        in_char = !in_char;
                    }
                }
                '\\' => {
                    if last_escape {
                        last_arg.push('\\');
                        last_escape = false;
                    } else {
                        last_escape = true;
                    }
                }
                ',' if !in_str && !in_char => {
                    result.push(last_arg.clone());
                    last_arg.clear();
                }
                ' ' if !in_str && !in_char => {}
                _ => last_arg.push(c),
            }
        }
        if last_arg != "" {
            result.push(last_arg.clone());
        }
        result.iter().map(|arg| arg.trim().to_string()).collect::<Vec<String>>()
    }
}

fn split_command(cmd: String) -> Vec<String> {
    let mut has_args = true; // Se o comando possui argumentos
    let index = match cmd.find(':') {
        Some(i) => i,
        None => {
            has_args = false;
            cmd.len()
        }
    };
    let cmd_name = &cmd[..index];
    let cmd_args: &str = if has_args {
        &cmd[index+1..]
    } else {
        ""
    };
    vec![cmd_name.to_string(), cmd_args.to_string()]
}

/// Faz parsing de um comando
fn parse_cmd(cmd: &str) -> Command {
    // Estrutura de um comando:
    // COMANDO: var1, var2, ...
    let cmd_parts = split_command(cmd.to_string());
    let cmd_parts = cmd_parts.iter().map(|part| part.trim()).collect::<Vec<&str>>();
    // argumentos
    let mut arguments: Vec<String> = Vec::new();
    // Tipo/nome do comando
    let cmd_type = if cmd_parts.len() > 1 {
        if n_of_char(',', cmd_parts[1]) == 0 {
            if cmd_parts[1].trim() != "" {
                // Um argumento
                arguments.push(cmd_parts[1].trim().to_string());
            }
        } else {
            arguments = split_arguments(cmd_parts[1].trim().to_string());
        }
        cmd_parts[0]
    } else {
        cmd.trim()
    };
    let cmd: Command = match cmd_type {
        kw::KW_MOVE => {
            check_n_params(CommandType::Move, arguments.len());
            let (addr1, addr2) = (arguments[0].clone(), arguments[1].clone());
            Command::Move(addr1, addr2)
        }
        kw::KW_CLEAR => {
            check_n_params(CommandType::Clear, arguments.len());
            Command::Clear(arguments[0].clone())
        }
        kw::KW_DECL => {
            check_n_params(CommandType::Decl, arguments.len());
            Command::Decl(arguments[0].clone())
        }
        kw::KW_DECLWV => {
            check_n_params(CommandType::DeclWV, arguments.len());
            let (name, val) = (arguments[0].clone(), arguments[1].clone());
            Command::DeclWV(name, val)
        }
        kw::KW_JUMP => {
            check_n_params(CommandType::Jump, arguments.len());
            Command::Jump(arguments[0].clone())
        }
        kw::KW_CMP => {
            check_n_params(CommandType::Cmp, arguments.len());
            let (addr1, addr2) = (arguments[0].clone(), arguments[1].clone());
            Command::Cmp(addr1, addr2)
        }
        kw::KW_CMP_EQ => {
            check_n_params(CommandType::CmpEq, arguments.len());
            Command::CmpEq(arguments[0].to_string())
        }
        kw::KW_CMP_NEQ => {
            check_n_params(CommandType::CmpNEq, arguments.len());
            Command::CmpNEq(arguments[0].to_string())
        }
        kw::KW_CMP_LESS => {
            check_n_params(CommandType::CmpLess, arguments.len());
            Command::CmpLess(arguments[0].to_string())
        }
        kw::KW_CMP_LESSEQ => {
            check_n_params(CommandType::CmpLessEq, arguments.len());
            Command::CmpLessEq(arguments[0].to_string())
        }
        kw::KW_CMP_MORE => {
            check_n_params(CommandType::CmpMore, arguments.len());
            Command::CmpMore(arguments[0].to_string())
        }
        kw::KW_CMP_MOREEQ => {
            check_n_params(CommandType::CmpMoreEq, arguments.len());
            Command::CmpMoreEq(arguments[0].to_string())
        }
        kw::KW_PRINTLN => {
            check_n_params(CommandType::Println, arguments.len());
            Command::Println(arguments.iter().map(|arg| arg.to_string()).collect::<Vec<String>>())
        }
        kw::KW_PRINT => {
            check_n_params(CommandType::Print, arguments.len());
            Command::Print(arguments.iter().map(|arg| arg.to_string()).collect::<Vec<String>>())
        }
        kw::KW_QUIT => {
            check_n_params(CommandType::Quit, arguments.len());
            Command::Quit
        }
        kw::KW_INPUT => {
            check_n_params(CommandType::Input, arguments.len());
            Command::Input(arguments[0].to_string())
        }
        kw::KW_INPUT_UP => {
            check_n_params(CommandType::InputUpper, arguments.len());
            Command::InputUpper(arguments[0].to_string())
        }
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
