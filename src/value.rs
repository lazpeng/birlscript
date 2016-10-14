//! Responsável pelo parsing de expressões

/// Biblioteca do parser de expressões
extern crate meval;

/// Resultado de uma expressão
#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    Str(Box<String>),
}

use std::fmt;

impl fmt::Display for Value {
    /// Função que torna possivel printar values diretamente
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Number(x) => write!(f, "{}", x),
            &Value::Str(ref x) => write!(f, "{}", x),
        }
    }
}

impl Value {
    /// Retorna uma representação em string do valor atual
    pub fn as_str(&self) -> String {
        let fmted = match self {
            &Value::Number(x) => format!("{}", x),
            &Value::Str(ref x) => format!("\"{}\"", x),
        };
        String::from(fmted)
    }

    /// Retorna o tipo do valor
    pub fn value_type(&self) -> ValueType {
        match *self {
            Value::Number(_) => ValueType::Number,
            Value::Str(_) => ValueType::Str,
        }
    }
}

use vm;

/// Expande os simbolos do ambiente atual para seus valores
fn expand_syms(expr: &mut String, env: &mut vm::VM) {
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
                        let var_val = env.retrieve_variable(&sym);
                        newexpr.push_str(&var_val.as_str());
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
            let var = env.retrieve_variable(&sym);
            newexpr.push_str(&var.as_str());
            sym.clear();
        }
        expr.clear();
        expr.push_str(&newexpr);
    } else {
        panic!("Expressão vazia!")
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Tipo do valor a ser interpretado
pub enum ValueType {
    Number,
    Str,
}

impl ValueType {
    /// Tenta identificar um ValueType apartir de uma string
    pub fn try_parse(expr: &str) -> Option<ValueType> {
        match expr.trim() {
            VALUETYPE_STR => Some(ValueType::Str),
            VALUETYPE_NUM => Some(ValueType::Number),
            _ => None,
        }
    }
}

/// Nome que identifica o tipo Str
pub const VALUETYPE_STR: &'static str = "FIBRA";
/// Nome que identifica o tipo Number
pub const VALUETYPE_NUM: &'static str = "TRAPEZIO DESCENDENTE";

/// Descobre o tipo de uma expressão
fn expr_type(expr: &str) -> ValueType {
    if expr == "" {
        panic!("Expressão vazia!")
    }
    // Tenta descobrir o tipo da expressão por meio dos seus primeiros caracteres
    let mut chars = expr.chars();
    match chars.nth(0).unwrap() {
        '0'...'9' => ValueType::Number,
        '-' => {
            match chars.next()
                .expect("Operador - aplicado em \"nada\"") {
                '0'...'9' => ValueType::Number,
                _ => panic!("Operador \"-\" atribuido a uma expressão que não o suporta."),
            }
        }
        '\'' | '\"' => ValueType::Str,
        _ => panic!("Tipo de expressão invalido. Expressão: {}", expr),
    }
}

/// Faz parsing de um numero
fn parse_num(expr: &str) -> Value {
    if expr.contains('\"') || expr.contains('\'') {
        panic!("Uma expressão com números não deve conter strings ou caracteres")
    }
    let res = match meval::eval_str(expr) {
        Ok(x) => x,
        Err(_) => 0.0,
    };
    Value::Number(res)
}

/// Separa uma expressão de Strings em varios tokens
fn parse_str_tokenize(expr: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![String::new()];
    let mut index = 0;
    let (mut in_str, mut last_escape, mut last_op) = (false, false, true); // Se esta no meio do parsing de uma string, se o ultimo foi escape e se o ultimo foi operador
    let mut in_char = false;
    for c in expr.chars() {
        match c {
            '\\' if in_str => {
                if last_escape {
                    tokens[index].push_str("\\");
                    last_escape = false;
                } else {
                    last_escape = true;
                }
            }
            '\"' if in_str => {
                if last_escape {
                    tokens[index].push_str("\"");
                    last_escape = false;
                } else {
                    in_str = false;
                }
            }
            '\"' if !in_str => {
                if !last_op {
                    panic!("No meio de duas strings so deve haver um operador! \
                                           expr: {}",
                           expr)
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
                    panic!("No meio de uma string e um caractere so deve haver um operador!")
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
            '-' | '*' | '/' if !in_str => panic!("O operador {} não é permitido em strings!", c),
            _ if in_char => {
                tokens[index].push(c);
            }
            '0'...'9' if !in_str && !in_char => {
                panic!("Números não devem ser usados em operações com strings ou caracteres. \
                        expr: {}",
                       expr)
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
        Value::Str(Box::new(tokens[0].clone()))
    } else {
        let mut result = String::new();
        for token in tokens {
            result.push_str(&token);
        }
        Value::Str(Box::new(result))
    }
}

/// Faz o parsing de uma expressão
pub fn parse_expr(expr: &str, env: &mut vm::VM) -> Value {
    let mut nexp = expr.trim().to_string();
    expand_syms(&mut nexp, env);
    match expr_type(&nexp) {
        ValueType::Number => parse_num(&nexp),
        ValueType::Str => parse_str(&nexp),
    }
}
