#![allow(dead_code)]

pub static NAME: &'static str = "BIRL Interpreter";
pub static VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub static GREETING: &'static str = "Aqui nóis constrói fibra.";

type Identifier = String;

use std::collections::HashMap;
#[derive(Clone, PartialEq, Debug)]
pub struct Structure {
    fields: HashMap<Identifier, Value>,
}
impl Structure {
    pub fn new() -> Structure{
        Structure{ fields: HashMap::new() }
    }

    pub fn size_of(&self) -> usize {
        // Add together all the entry sizes
        self.fields.values().fold(0, |size, entry| size + entry.size_of())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Function{ function: Box<i8> }
impl Function {
    pub fn call() -> Type {
        Type::Void
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Type{
    Array, Str, Char, Struct, Void,
    Unsigned8, Unsigned16, Unsigned32, Unsigned64, UnsignedSize,
      Signed8,   Signed16,   Signed32,   Signed64,   SignedSize,
    Float32, Float64
}
impl Type{
    pub fn default(self) -> Value{
        match self{
            Type::Array  => Value::Array(Vec::new()),
            Type::Str    => Value::Str(String::new()),
            Type::Char   => Value::Char(' '),
            Type::Struct => Value::Struct(Structure::new()),
            Type::Void   => Value::Void,

            Type::Unsigned8    => Value::Unsigned8(0),    Type::Signed8    => Value::Signed8(0),
            Type::Unsigned16   => Value::Unsigned16(0),   Type::Signed16   => Value::Signed16(0),
            Type::Unsigned32   => Value::Unsigned32(0),   Type::Signed32   => Value::Signed32(0),
            Type::Unsigned64   => Value::Unsigned64(0),   Type::Signed64   => Value::Signed64(0),
            Type::UnsignedSize => Value::UnsignedSize(0), Type::SignedSize => Value::SignedSize(0),

            Type::Float32 => Value::Float32(0.0),
            Type::Float64 => Value::Float64(0.0),
        }
    }
}

/// Possible language types, and their correspondants in Rust
#[derive(Clone, PartialEq, Debug)]
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

    pub fn try_cast(self, target: Type) -> Result<Value, Value>{
        macro_rules! cast_number{
            ($num:ident) => {
                match target{
                    // Cast to other number types
                    Type::Unsigned8 => Ok(Value::Unsigned8($num as u8)), Type::Signed8 => Ok(Value::Signed8($num as i8)),
                    Type::Unsigned16 => Ok(Value::Unsigned16($num as u16)), Type::Signed16 => Ok(Value::Signed16($num as i16)),
                    Type::Unsigned32 => Ok(Value::Unsigned32($num as u32)), Type::Signed32 => Ok(Value::Signed32($num as i32)),
                    Type::Unsigned64 => Ok(Value::Unsigned64($num as u64)), Type::Signed64 => Ok(Value::Signed64($num as i64)),
                    Type::UnsignedSize => Ok(Value::UnsignedSize($num as usize)), Type::SignedSize => Ok(Value::SignedSize($num as isize)),

                    Type::Float32 => Ok(Value::Float32($num as f32)), Type::Float64 => Ok(Value::Float64($num as f64)),

                    // Format onto a String
                    Type::Str => Ok(Value::Str(format!("{}", $num))),

                    // No other valid conversions
                    _ => { Err(self) }
                }
            }
        }

        if let Type::Void = target {
            Ok(Value::Void)
        }else{
            match self{
                Value::Char(c) => if let Type::Str = target { Ok(Value::Str(c.escape_unicode().collect())) } else { Err(self) },

                // Implement scalar casting for all number types
                Value::Unsigned8(number)  => cast_number!(number), Value::Signed8(number) => cast_number!(number),
                Value::Unsigned16(number) => cast_number!(number), Value::Signed16(number) => cast_number!(number),
                Value::Unsigned32(number) => cast_number!(number), Value::Signed32(number) => cast_number!(number),
                Value::Unsigned64(number) => cast_number!(number), Value::Signed64(number) => cast_number!(number),
                Value::UnsignedSize(number) => cast_number!(number), Value::SignedSize(number) => cast_number!(number),
                Value::Float32(number) => cast_number!(number), Value::Float64(number) => cast_number!(number),

                _ => { Err(self) }
            }
        }
    }
}

#[test]
fn value(){
    // Casting
    let value = Value::Signed64(-159357789456123);

    // Stingifying
    assert_eq!(value.clone().try_cast(Type::Str).unwrap(), Value::Str("-159357789456123".to_owned()));

    // Casting between signed and unsigned
    assert_eq!(value.clone().try_cast(Type::Unsigned64).unwrap(), Value::Unsigned64(18446584715920095493));
}

#[derive(Clone, PartialEq, Debug)]
pub struct Context{
    variables: HashMap<Identifier, Value>,
    functions: HashMap<Identifier, Function>,

    stack: Vec<Context>
}
impl Context{
    /// Saves the current state of the context
    pub fn push(&mut self){
        let copy = self.clone();
        self.stack.push(copy);
    }

    /// Revers to a previously saved context discarding all changes
    pub fn pop(&mut self){
        let previous = self.stack.pop().expect("Tried to pop Context with empty stack");
        *self = previous
    }

    /// Reverts to a previously saved context retaining the changes made to values present in it
    pub fn merge_pop(&mut self){
        let mut previous = self.stack.pop().expect("Tried to pop Context with empty stack");

        for (key, value) in previous.variables.iter_mut(){
            // Apply changes made to variables present on the previous stack
            if let Some(new) = self.variables.remove(key){ *value = new }
        }
        for (key, value) in previous.functions.iter_mut(){
            // Apply changes made to functions present on the previous stack
            if let Some(new) = self.functions.remove(key){ *value = new }
        }

        *self = previous
    }
}

use std::rc::Rc;
use std::cell::RefCell;
pub struct VirtualMachine{
    memory: Vec<Rc<RefCell<Value>>>
}
impl VirtualMachine{
    pub fn new() -> VirtualMachine{
        VirtualMachine{ memory: Vec::new() }
    }
}
