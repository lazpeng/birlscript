#![allow(dead_code)]

pub static NAME: &'static str = "BIRL Interpreter";
pub static VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub static GREETING: &'static str = "Aqui nóis constrói fibra.";

type Identifier = String;

use std::collections::HashMap;
pub struct Structure {
    fields: HashMap<Identifier, Value>,
}
impl Structure {
    pub fn size_of(&self) -> usize {
        // Add together all the entry sizes
        self.fields.values().fold(0, |size, entry| size + entry.size_of())
    }
}

pub enum Type{
    Array, Str, Char, Struct, Void,
    Unsigned8, Unsigned16, Unsigned32, Unsigned64, UnsignedSize,
      Signed8,   Signed16,   Signed32,   Signed64,   SignedSize,
    Float32, Float64
}
impl Type{

}

/// Possible language types, and their correspondants in Rust
pub enum Value {
    /// Stores a collections of other elements
    Array(Vec<Value>),
    /// Owned string
    Str(String),
    /// Character
    Char(char),
    /// Structure
    Struct(Structure),
    /// A type containing no value
    Void,

    // Integer types
    Unsigned8(u8), Unsigned16(u16), Unsigned32(u32), Unsigned64(u64), UnsignedSize(usize),
      Signed8(i8), Signed16(i16),   Signed32(i32),   Signed64(i64),   SignedSize(isize),

    // Floats
    Float32(f32), Float64(f64),
}
impl Value {
    pub fn get_type(&self) -> Type{
        match self{
            &Value::Array(_) => Type::Array,
            &Value::Str(_) => Type::Str,
            &Value::Char(_) => Type::Char,
            &Value::Struct(_) => Type::Struct,

            &Value::Void => Type::Void,

            // Both signed and unsigned types have the same size in bytes
            &Value::Unsigned8(_)    => Type::Unsigned8,    &Value::Signed8(_)    => Type::Signed8,
            &Value::Unsigned16(_)   => Type::Unsigned16,   &Value::Signed16(_)   => Type::Signed16,
            &Value::Unsigned32(_)   => Type::Unsigned32,   &Value::Signed32(_)   => Type::Signed32,
            &Value::Unsigned64(_)   => Type::Unsigned64,   &Value::Signed64(_)   => Type::Signed64,
            &Value::UnsignedSize(_) => Type::UnsignedSize, &Value::SignedSize(_) => Type::SignedSize,

            &Value::Float32(_) => Type::Float32,
            &Value::Float64(_) => Type::Float64,
        }
    }

    pub fn size_of(&self) -> usize {
        use std::mem::size_of as len;
        match self {
            &Value::Array(ref array) => array.iter().fold(0, |size, entry| size + entry.size_of()),
            &Value::Str(ref string) => string.len(),
            &Value::Char(ref c) => c.len_utf8(),
            &Value::Struct(ref structure) => structure.size_of(),

            // Internally, voids have no size
            &Value::Void => 0,

            // Both signed and unsigned types have the same size in bytes
            &Value::Unsigned8(_) | &Value::Signed8(_) => len::<u8>(),
            &Value::Unsigned16(_) | &Value::Signed16(_) => len::<u16>(),
            &Value::Unsigned32(_) | &Value::Signed32(_) => len::<u32>(),
            &Value::Unsigned64(_) | &Value::Signed64(_) => len::<u64>(),
            &Value::UnsignedSize(_) | &Value::SignedSize(_) => len::<usize>(),

            &Value::Float32(_) => len::<f32>(),
            &Value::Float64(_) => len::<f64>(),
        }
    }
}

pub struct Function {
    name: String,
}
impl Function {
    pub fn call() -> Type {
        Type::Void
    }
}
