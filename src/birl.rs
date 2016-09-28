#![allow(dead_code)]

pub static NAME: &'static str = "BIRL Interpreter";
pub static VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub static GREETING: &'static str = "Aqui nóis constrói fibra.";

pub type Identifier = String;

/// Wraps a function that is executed
pub trait Function{
    /// Expected arguments in order
    fn arguments(&self) -> Vec<Template>;
    /// Run the function with the given arguments
    fn run(&mut self, Vec<Value>) -> Value;
}

use std::collections::HashMap;
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Template{
    Array, Str, Char, Struct(HashMap<Identifier, Template>), Void,
    Unsigned8, Unsigned16, Unsigned32, Unsigned64, UnsignedSize,
      Signed8,   Signed16,   Signed32,   Signed64,   SignedSize,
    Float32, Float64
}
impl Template{
    /// Creates a new Value using defaults
    pub fn default(self) -> Value{
        match self{
            Template::Array => Value::Array(Vec::new()),
            Template::Str   => Value::Str(String::new()),
            Template::Char  => Value::Char(' '),
            Template::Struct(values) => Value::Struct(values.into_iter().map(|(key, template)| (key, template.default())).collect()),
            Template::Void  => Value::Void,

            Template::Unsigned8 => Value::Unsigned8(0), Template::Signed8 => Value::Signed8(0),
            Template::Unsigned16 => Value::Unsigned16(0), Template::Signed16 => Value::Signed16(0),
            Template::Unsigned32 => Value::Unsigned32(0), Template::Signed32 => Value::Signed32(0),
            Template::Unsigned64 => Value::Unsigned64(0), Template::Signed64 => Value::Signed64(0),
            Template::UnsignedSize => Value::UnsignedSize(0), Template::SignedSize => Value::SignedSize(0),

            Template::Float32 => Value::Float32(0.0), Template::Float64 => Value::Float64(0.0)
        }
    }
}

/// Declared values containing concrete data for templates
#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    /// Stores a collections of other elements
    Array(Vec<Value>),
    /// Owned string
    Str(String),
    /// Character
    Char(char),
    /// Structure
    Struct(HashMap<Identifier, Value>),
    /// A type containing no value
    Void,

    // Integer types
    Unsigned8(u8), Unsigned16(u16), Unsigned32(u32), Unsigned64(u64), UnsignedSize(usize),
      Signed8(i8), Signed16(i16),   Signed32(i32),   Signed64(i64),   SignedSize(isize),

    // Floats
    Float32(f32), Float64(f64),
}
impl Value {
    pub fn get_type(&self) -> Template{
        match self{
            &Value::Array(_) => Template::Array,
            &Value::Str(_) => Template::Str,
            &Value::Char(_) => Template::Char,
            &Value::Struct(ref s) => Template::Struct(
                s.iter().map(|(key, value)| (key.clone(), value.get_type())).collect::<HashMap<_, _>>()
            ),

            &Value::Void => Template::Void,

            // Both signed and unsigned types have the same size in bytes
            &Value::Unsigned8(_)    => Template::Unsigned8,    &Value::Signed8(_)    => Template::Signed8,
            &Value::Unsigned16(_)   => Template::Unsigned16,   &Value::Signed16(_)   => Template::Signed16,
            &Value::Unsigned32(_)   => Template::Unsigned32,   &Value::Signed32(_)   => Template::Signed32,
            &Value::Unsigned64(_)   => Template::Unsigned64,   &Value::Signed64(_)   => Template::Signed64,
            &Value::UnsignedSize(_) => Template::UnsignedSize, &Value::SignedSize(_) => Template::SignedSize,

            &Value::Float32(_) => Template::Float32,
            &Value::Float64(_) => Template::Float64,
        }
    }

    pub fn size_of(&self) -> usize {
        use std::mem::size_of as len;
        match self {
            &Value::Array(ref array) => array.iter().fold(0, |size, entry| size + entry.size_of()),
            &Value::Str(ref string) => string.len(),
            &Value::Char(ref c) => c.len_utf8(),
            &Value::Struct(ref structure) => structure.values().fold(0, |size, entry| size + entry.size_of()),

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

    pub fn try_cast(self, target: Template) -> Result<Value, Value>{
        macro_rules! cast_number{
            ($num:ident) => {
                match target{
                    // Cast to other number types
                    Template::Unsigned8 => Ok(Value::Unsigned8($num as u8)), Template::Signed8 => Ok(Value::Signed8($num as i8)),
                    Template::Unsigned16 => Ok(Value::Unsigned16($num as u16)), Template::Signed16 => Ok(Value::Signed16($num as i16)),
                    Template::Unsigned32 => Ok(Value::Unsigned32($num as u32)), Template::Signed32 => Ok(Value::Signed32($num as i32)),
                    Template::Unsigned64 => Ok(Value::Unsigned64($num as u64)), Template::Signed64 => Ok(Value::Signed64($num as i64)),
                    Template::UnsignedSize => Ok(Value::UnsignedSize($num as usize)), Template::SignedSize => Ok(Value::SignedSize($num as isize)),

                    Template::Float32 => Ok(Value::Float32($num as f32)), Template::Float64 => Ok(Value::Float64($num as f64)),

                    // Format onto a String
                    Template::Str => Ok(Value::Str(format!("{}", $num))),

                    // No other valid conversions
                    _ => { Err(self) }
                }
            }
        }

        if let Template::Void = target {
            Ok(Value::Void)
        }else{
            match self{
                Value::Char(c) => if let Template::Str = target { Ok(Value::Str(c.escape_unicode().collect())) } else { Err(self) },

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
    // Stingifying
    assert_eq!(Value::Signed64(-159357789456123).try_cast(Template::Str).unwrap(), Value::Str("-159357789456123".to_owned()));

    // Casting between signed and unsigned
    assert_eq!(Value::Signed64(-159357789456123).try_cast(Template::Unsigned64).unwrap(), Value::Unsigned64(18446584715920095493));
}

/// Represents a variable that has botah a template and a corresponding value,
/// which should be of the same type and may or may not have been defined. If
/// it has not been declared, `self.value` will be None.
pub struct Variable{
    pub template: Template,
    pub value: Option<Value>
}
impl Variable{
    /// Tries unwrapping the variable's value
    pub fn unwrap(self) -> Value{ self.value.unwrap() }
}

pub struct FunctionPointer<'a>{
    arguments: Vec<Template>,
    function:  &'a mut FnMut(&mut Scope, Vec<Value>) -> Value
}
impl<'a> FunctionPointer<'a> {
    pub fn call(&mut self, scope: &mut Scope, arguments: Vec<Value>) -> Value {
        // Push scope
        scope.push();
        let func = &mut self.function;
        func(scope, arguments)
    }
}

pub struct Scope<'a>{
    variables: HashMap<Identifier, Value>,
    functions: HashMap<Identifier, Box<Function + 'a>>,
    parent: Option<Box<Scope<'a>>>
}
impl<'a> Scope<'a>{
    /// Creates a new root scope, I.E., a scope with no parent scopes.
    pub fn new() -> Scope<'a>{
        Scope{
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None
        }
    }

    /// Swaps self for an uninitialized value.
    /// NOTE: This is an unsafe operation, which MUST be undone before returning.
    unsafe fn _uninitialize(&mut self) -> Scope<'a>{
        use std::mem;
        mem::replace(self, mem::uninitialized())
    }

    /// Sets self to the given scope, deleting the previous value
    fn _set(&mut self, value: Scope<'a>){
        use std::mem;
        mem::replace(self, value);
    }

    /// Creates an empt child scope, with self as its parent
    pub fn push(&mut self){
        unsafe{
            let original = self._uninitialize();
            self._set(Scope{
                variables: HashMap::new(),
                functions: HashMap::new(),
                parent: Some(Box::new(original))
            });
        }
    }

    /// Reverts to the previously saved scope, discarding any locally declared variables.
    /// # Panics
    /// Panics when no parent scope is available, in other words, if this is the root scope.
    pub fn pop(&mut self){
        unsafe{
            let parent = self._uninitialize().parent.expect("Tried to pop Context with empty stack");
            self._set(*parent);
        }
    }

    pub fn function(&'a mut self, identifier: &Identifier) -> Option<&'a mut Function>{
        match self.functions.get_mut(identifier){
            // Found a function locally
            Some(local) => Some(local.as_mut()),

            // Try to find it in a parent scope, if any are available
            None => match &mut self.parent{
                &mut Some(ref mut parent) => parent.function(identifier).map(|pointer| pointer),
                &mut None => None
            }
        }
    }

    pub fn variable(&'a self, identifier: &Identifier) -> Option<&'a Value>{
        match self.variables.get(identifier){
            // Found a variable locally
            Some(local) => Some(local),

            // Try to find it in a parent scope, if any are available
            None => match &self.parent{
                &Some(ref parent) => parent.variable(identifier),
                &None => None
            }
        }
    }

    pub fn variable_mut(&'a mut self, identifier: &Identifier) -> Option<&'a mut Value>{
        match self.variables.get_mut(identifier){
            // Found a variable locally
            Some(local) => Some(local),

            // Try to find it in a parent scope, if any are available
            None => match &mut self.parent{
                &mut Some(ref mut parent) => parent.variable_mut(identifier),
                &mut None => None
            }
        }
    }

    pub fn declare(&mut self, identifier: Identifier, value: Value){
        self.variables.insert(identifier, value);
    }
}

#[test]
fn scope(){
    let mut scope = Scope::new();

    scope.declare("ValorLegal".to_owned(), Value::Str("10/09/2015".to_owned()));
    scope.declare("ValorLegalIIAVingança".to_owned(), Value::Str("20:00".to_owned()));

    assert_eq!(scope.variable(&"ValorLegal".to_owned()).unwrap().clone(), Value::Str("10/09/2015".to_owned()));
    assert_eq!(scope.variable(&"ValorLegalIIAVingança".to_owned()).unwrap().clone(), Value::Str("20:00".to_owned()));
}
