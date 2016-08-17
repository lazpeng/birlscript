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

/// Representa um valor que pode ser atribuido a uma variavel
pub enum Value {
    /// Numero inteiro de 64 bits
    Integer(i64),
    /// Numero de ponto flutuante de 64 bits
    FloatP(f64),
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
                'a' ... 'z' | 'A' ... 'Z' | '_' => {
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
    Move(u64, u64),
    /// Limpa o valor da variavel no endereco a
    Clear(u64),
    /// Aplica xor na variavel no endereco a com o valor b
    Xor(u64, i64),
    /// Aplica and na variavel no endereco a com o valor b
    And(u64, i64),
    /// Aplica or  na variavel no endereco a com o valor b
    Or(u64, i64),
    /// Adiciona b ao valor da variavel no endereco a
    Add(u64, i64),
    /// Remove b do valor da variavel no endereco a
    Rem(u64, i64),
    /// Divide o valor da variavel no endereco a com o valor b
    Div(u64, i64),
    /// Multiplica o valor da variavel no endereco a com o valor b
    Mul(u64, i64),
    /// Multiplica um valor por -1
    Neg(u64),
    /// Declara a variavel com nome a
    Decl(String),
    /// Declara a variavel com nome a e valor b
    DeclWV(String, Value),
    /// Passa a execução para outra seção com nome a, retornando uma instrução à frente
    Jump(String),
    /// Compara os valores de a e b, usado em condicionais
    Cmp(u64, u64),
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
        arguments = cmd_parts[1].split(',').collect::<Vec<&str>>();
    }
    match cmd_type {
        kw::KW_MOVE => {}
        kw::KW_CLEAR => {}
        kw::KW_XOR => {}
        kw::KW_AND => {}
        kw::KW_OR => {}
        kw::KW_ADD => {}
        kw::KW_REM => {}
        kw::KW_DIV => {}
        kw::KW_MUL => {}
        kw::KW_DECL => {}
        kw::KW_DECLWV => {}
        kw::KW_JUMP => {}
        kw::KW_CMP => {}
        _ => panic!("Erro: Tipo de comando \"{}\" não existe.", cmd_type),
    }
    None
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
    unimplemented!();
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
            let line = lines[l];
            sect.lines.push(parse_cmd(line).unwrap());
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
