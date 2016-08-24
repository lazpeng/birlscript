//! Responsável pelo parsing de expressões

/// Biblioteca do parser de expressões
extern crate meval;

// O parsing de expressões deve ocorrer em tempo de execução para que se faça uso das variaveis

/// Resultado de uma expressão
pub enum Value {
    Int(i64),
    Float(f64),
    Char(char),
    Str(String),
}

/// Faz o parsing de uma expressão
pub fn parse_expr(expr: &str) -> Value {
    unimplemented!()
}
