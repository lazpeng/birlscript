//! Modulo de evaluação de valores

#[cfg(target_pointer_width = "64")]
pub type NumericType = f64;

#[cfg(target_pointer_width = "32")]
pub type NumericType = f32;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Str(String),
    Num(NumericType),
    Structure, // Futuro
    NullOrEmpty,
}

pub trait ValueQuery {
    fn query(id: &str) -> Value;

    fn query_raw(id: &str) -> String;
}

pub fn evaluate<'a, QueryObj>(expression: &str, query_func: &'a QueryObj) -> Value
    where QueryObj: ValueQuery + 'a
{
    unimplemented!()
}

pub fn evaluate_raw<'a, QueryObj>(expression: &str, query_func: &'a QueryObj) -> Value
    where QueryObj: ValueQuery + 'a
{
    unimplemented!()
}
