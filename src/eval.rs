//! Modulo de evaluação de valores

use super::meval;
use std::fmt::{Display, Formatter, self};

#[cfg(target_pointer_width = "64")]
pub type NumericType = f64;

#[cfg(target_pointer_width = "32")]
pub type NumericType = f32;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Str(String),
    Num(NumericType),
    NullOrEmpty,
}

impl Value {
    pub fn as_string(&self) -> String {
        match self {
            &Value::Str(ref v) => {
                let mut result = String::from("\"");
                result.push_str(v);
                result.push('\"');
                result
            }
            &Value::Num(v) => v.to_string(),
            &Value::NullOrEmpty => String::new(),
        }
    }

    pub fn value_type(&self) -> ValueType {
        match self {
            &Value::Str(_) => ValueType::Str,
            &Value::Num(_) => ValueType::Num,
            &Value::NullOrEmpty => ValueType::NullOrEmpty,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Value::Num(x) => write!(f, "{}", x),
            &Value::Str(ref x) => write!(f, "{}", x),
            &Value::NullOrEmpty => write!(f, ""), // Empty value
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Tipo do valor a ser interpretado
pub enum ValueType {
    Num,
    Str,
    NullOrEmpty,
}

impl ValueType {
    /// Tenta identificar um ValueType apartir de uma string
    pub fn try_parse(expr: &str) -> Option<ValueType> {
        match expr.trim() {
            VALUETYPE_STR => Some(ValueType::Str),
            VALUETYPE_NUM => Some(ValueType::Num),
            _ => None,
        }
    }
}

pub trait ValueQuery {
    fn query(&self, id: &str) -> Option<Value>;

    fn query_raw(&self, id: &str) -> Option<String>;
}

fn expand_symbols<'a, QObj>(expression: &mut String, query_obj: &'a QObj, use_raw: bool)
    where QObj: ValueQuery
{
    if !expression.is_empty() {
        // Se esta no meio de uma string
        let mut is_str = false;
        // Se o ultimo caractere foi de escape
        let mut last_escape = false;
        // O ultimo simbolo, se esta no meio de um simbolo e se esta no meio de um caractere
        let (mut sym, mut is_sym, mut is_char) = (String::new(), false, false);
        // A nova string
        let mut newexpr = String::new();
        for c in expression.chars() {
            if is_sym {
                match c {
                    ' ' | '+' | '-' | '/' | '*' | '&' | '|' | '%' => {
                        is_sym = false;
                        let var_val;
                        if use_raw {
                            var_val = query_obj.query_raw(&sym).unwrap_or_else(|| panic!("Erro na expansão da expressão, simbolo não expandido. expr: {}", expression));
                        } else {
                            var_val = query_obj.query(&sym).unwrap_or_else(|| panic!("Erro na expansão da expressão, simbolo não expandido. expr: {}", expression)).as_string();
                        }
                        newexpr.push_str(&var_val);
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
            let var_val;
            if use_raw {
                var_val = query_obj.query_raw(&sym).unwrap_or_else(|| panic!("Erro na expansão da expressão, simbolo não expandido. expr: {}", expression));
            } else {
                var_val = query_obj.query(&sym).unwrap_or_else(|| panic!("Erro na expansão da expressão, simbolo não expandido. expr: {}", expression)).as_string();
            }
            newexpr.push_str(&var_val);
            sym.clear();
        }
        expression.clear();
        expression.push_str(&newexpr);
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
    match chars.next().unwrap() { // Expressão garantida a ter ao menos um caractere
        '0'...'9' => ValueType::Num,
        '-' => {
            match chars.next()
                .expect("Operador - aplicado em \"nada\"") {
                '0'...'9' => ValueType::Num,
                _ => panic!("Operador \"-\" atribuido a uma expressão que não o suporta."),
            }
        }
        '\'' | '\"' => ValueType::Str,
        _ => panic!("Tipo de expressão invalido. Expressão: {}", expr),
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
    let mut tokens = parse_str_tokenize(expr);
    // Se há multiplas strings, é porque foi usado o operador +, se nao houve um erro
    if tokens.len() == 1 {
        // Só há uma string
        Value::Str(tokens.remove(0))
    } else {
        let mut result = String::new();
        for token in tokens {
            result.push_str(&token);
        }
        Value::Str(result)
    }
}

/// Faz parsing de um numero
fn parse_num(expr: &str) -> Value {
    if expr.contains('\"') || expr.contains('\'') {
        panic!("Uma expressão com números não deve conter strings ou caracteres")
    }
    let res: NumericType = match meval::eval_str(expr) {
        Ok(x) => x as NumericType, // Se for f32, converte
        Err(_) => 0.0,
    };
    Value::Num(res)
}

/// Faz a evaluação de uma expressão e retorna pelo Enum correspondente
pub fn evaluate<'a, QueryObj>(expression: &str, query_func: &'a QueryObj) -> Value
    where QueryObj: ValueQuery + 'a
{
    let mut expr = expression.trim().to_owned();
    expand_symbols(&mut expr, query_func, false);
    match expr_type(&expr) {
            ValueType::Num => parse_num(&expr),
            ValueType::Str => parse_str(&expr),
            ValueType::NullOrEmpty => Value::NullOrEmpty,
    }
}

/// Faz a evaluação dos valores recebendo-os em forma de String e retorna também na forma de String
pub fn evaluate_raw<'a, QueryObj>(expression: &str, query_func: &'a QueryObj) -> String
    where QueryObj: ValueQuery + 'a
{
    let mut expr = expression.trim().to_owned();
    expand_symbols(&mut expr, query_func, true);
    let result = match expr_type(&expr) {
            ValueType::Num => parse_num(&expr),
            ValueType::Str => parse_str(&expr),
            ValueType::NullOrEmpty => Value::NullOrEmpty,
    };
    result.as_string()
}
