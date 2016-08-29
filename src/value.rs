//! Responsável pelo parsing de expressões

/// Biblioteca do parser de expressões
extern crate meval;

// O parsing de expressões deve ocorrer em tempo de execução para que se faça uso das variaveis

mod arch {
    #[cfg(target_pointer_width = "32")]
    pub type NumType = f32;

    #[cfg(target_pointer_width = "64")]
    pub type NumType = f64;
}

/// Resultado de uma expressão
#[derive(Clone)]
pub enum Value {
    Number(arch::NumType),
    Char(char),
    Str(String),
}

use std::fmt;
use error;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Number(x) => write!(f, "{}", x),
            &Value::Char(x) => write!(f, "{}", x),
            &Value::Str(ref x) => write!(f, "{}", x),
        }
    }
}

impl Value {
    fn as_str(&self) -> String {
        let fmted = match self {
            &Value::Number(x) => format!("{}", x),
            &Value::Char(x) => format!("'{}'", x),
            &Value::Str(ref x) => format!("\"{}\"", x),
        };
        String::from(fmted)
    }
}

use interpreter::{Environment, Variable};

/// Expande os simbolos do ambiente atual para seus valores
fn expand_syms(expr: &mut String, env: &mut Environment) {
    if expr != "" {
        // Se esta no meio de uma string
        let mut is_str = false;
        // Se o ultimo caractere foi de escape
        let mut last_escape = false;
        // O ultimo simbolo, se esta no meio de um simbolo e se esta no meio de um caractere
        let (mut sym, mut is_sym, mut is_char) = (String::new(), false, false);
        // A nova string
        let mut newexpr = String::new();
        for c in expr.chars() {
            if is_sym {
                match c {
                    ' ' | '+' | '-' | '/' | '*' | '&' | '|' | '%' => {
                        is_sym = false;
                        let var = match env.get_var(&sym) {
                            None => {
                                error::abort(&format!("Simbolo \"{}\" não reconhecido.", sym));
                                unreachable!()
                            }
                            Some(v) => v,
                        };
                        newexpr.push_str(&var.value.as_str());
                        newexpr.push(c);
                        sym.clear();
                    }
                    _ => sym.push(c),
                }
            } else {
                match c {
                    '\"' => {
                        if last_escape {
                            last_escape = false;
                        } else {
                            is_str = !is_str;
                        }
                    }
                    '\\' if is_str => {
                        last_escape = !last_escape;
                    }
                    'a'...'z' | 'A'...'Z' | '_' if !is_str && !is_char => {
                        is_sym = true;
                        sym.push(c);
                        continue;
                    }
                    '\'' if !is_str => is_char = !is_char,
                    _ => {}
                }
                newexpr.push(c);
            }
        }
        // Verifica se um simbolo ficou para traz
        if is_sym && sym != "" {
            let var: Variable = match env.get_var(&sym) {
                None => {
                    error::abort(&format!("Simbolo \"{}\" não reconhecido.", sym));
                    unreachable!()
                }
                Some(v) => v,
            };
            newexpr.push_str(&var.value.as_str());
            sym.clear();
        }
        expr.clear();
        expr.push_str(&newexpr);
    }
}

/// Tipo do valor a ser interpretado
enum ValueType {
    Number,
    Char,
    Str,
}

/// Descobre o tipo de uma expressão
fn expr_type(expr: &str) -> ValueType {
    if expr == "" {
        error::abort("Expressão vazia!");
    }
    // Tenta descobrir o tipo da expressão por meio dos seus primeiros caracteres
    let mut chars = expr.chars();
    match chars.nth(0).unwrap() {
        '0'...'9' => ValueType::Number,
        '-' => {
            match chars.nth(1).unwrap() {
                '0'...'9' => ValueType::Number,
                _ => {
                    error::abort("Operador \"-\" atribuido a uma expressão que não o suporta.");
                    unreachable!()
                }
            }
        }
        '\'' => ValueType::Char,
        '\"' => ValueType::Str,
        _ => {
            error::abort(&format!("Tipo de expressão invalido. Expressão: {}", expr));
            unreachable!()
        }
    }
}

/// Faz parsing de um numero
fn parse_num(expr: &str) -> Value {
    if expr.contains('\"') || expr.contains('\'') {
        error::abort("Uma expressão com números não deve conter strings ou caracteres");
    }
    let res: arch::NumType = meval::eval_str(expr).unwrap();
    Value::Number(res)
}

/// Faz parsing do valor de um caractere
fn parse_char(expr: &str) -> Value {
    // Numa expressão que possui apenas um caractere, nenhum operador é permitido
    let mut chars = expr.trim().chars();
    if expr.len() != 3 {
        // Um para o ', o valor e outro '
        error::abort(&format!("Erro na expressão do caractere: Numero incorreto de expressões: \
                               {}",
                              expr.len()));
        unreachable!() // abort, então esse codigo não será executado
    } else {
        Value::Char(chars.nth(1).unwrap())
    }
}

/// Separa uma expressão de Strings em varios tokens
fn parse_str_tokenize(expr: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![String::new()];
    let mut index = 0;
    let (mut in_str, mut last_escape, mut last_op) = (false, false, true); // Se esta no meio do parsing de uma string, se o ultimo foi escape e se o ultimo foi operador
    let mut in_char = false;
    for c in expr.chars() {
        match c {
            '\"' if in_str => {
                if last_escape {
                    tokens[index].push_str("\\\"");
                    last_escape = false;
                } else {
                    in_str = false;
                }
            }
            '\"' if !in_str => {
                if !last_op {
                    error::abort(&format!("No meio de duas strings so deve haver um operador! \
                                           expr: {}",
                                          expr));
                } else {
                    last_op = false;
                    in_str = true;
                    index += 1;
                    tokens.push(String::new());
                }
            }
            '\'' if !in_str && !in_char => {
                // Caractere
                if !last_op {
                    error::abort("No meio de uma string e um caractere so deve haver um operador!");
                }
                in_char = true;
                index += 1;
                tokens.push(String::new());
            }
            '\'' if in_char => {
                in_char = false;
            }
            '+' if !in_str => {
                last_op = true;
            }
            '-' | '*' | '/' if !in_str => {
                error::abort(&format!("O operador {} não é permitido em strings!", c));
            }
            _ if in_char => {
                tokens[index].push(c);
            }
            '0'...'9' if !in_str && !in_char => {
                error::abort("Números não devem ser usados em operações com strings ou caracteres");
            }
            _ if !in_str => {} // Pula outros caracteres se de fora de uma string
            _ => tokens[index].push(c),
        }
    }
    tokens
}

/// Faz parsing de um valor envolvendo strings
fn parse_str(expr: &str) -> Value {
    // Expressões de Strings podem usar o operador '+', usando apenas strings e caracteres
    let tokens = parse_str_tokenize(expr);
    // Se há multiplas strings, é porque foi usado o operador +, se nao houve um erro
    if tokens.len() == 1 {
        // Só há uma string
        Value::Str(tokens[0].clone())
    } else {
        let mut result = String::new();
        for token in tokens {
            result.push_str(&token);
        }
        Value::Str(result)
    }
}

/// Faz o parsing de uma expressão
pub fn parse_expr(expr: &str, env: &mut Environment) -> Value {
    let mut nexp = expr.trim().to_string();
    expand_syms(&mut nexp, env);
    match expr_type(&nexp) {
        ValueType::Number => parse_num(&nexp),
        ValueType::Char => parse_char(&nexp),
        ValueType::Str => parse_str(&nexp),
    }
}
