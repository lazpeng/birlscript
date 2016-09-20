pub static NAME:     &'static str = "BIRL Interpreter";
pub static VERSION:  &'static str = env!("CARGO_PKG_VERSION");
pub static GREETING: &'static str = "Aqui nóis constrói fibra.";

type Identifier = String;

use std::collections::HashMap;
pub struct Structure{
	fields: HashMap<Identifier, Type>
}
impl Structure{
	pub fn size_of(&self) -> usize{
		// Add together all the entry sizes
		self.fields.values().fold(0, |size, entry| size + entry.size_of())
	}
}

/// Possible language types, and their correspondants in Rust
pub enum Type{
	/// Stores a collections of other elements
	Array(Vec<Type>),
	/// Owned string
	Str(String),
	/// Character
	Char(char),
	/// Structure
	Struct(Structure),
	/// A type containing no value
	Void,


	/* Integer types */
	Unsigned8(u8), Unsigned16(u16), Unsigned32(u32), Unsigned64(u64), UnsignedSize(usize),
	  Signed8(i8),   Signed16(i16),   Signed32(i32),   Signed64(i64),   SignedSize(isize),

	/* Floats */
	Float32(f32), Float64(f64)
}
impl Type{
	pub fn size_of(&self) -> usize{
		use std::mem::size_of as len;
		match self{
			&Type::Array(ref array) => array.iter().fold(0, |size, entry| size + entry.size_of()),
			&Type::Str(ref string)  => string.len(),
			&Type::Char(ref c) => c.len_utf8(),
			&Type::Struct(ref structure) => structure.size_of(),

			// Internally, voids have no size
			&Type::Void => 0,

			// Both signed and unsigned types have the same size in bytes
			&Type::Unsigned8(_) | &Type::Signed8(_) => len::<u8>(),
			&Type::Unsigned16(_) | &Type::Signed16(_) => len::<u16>(),
			&Type::Unsigned32(_) | &Type::Signed32(_) => len::<u32>(),
			&Type::Unsigned64(_) | &Type::Signed64(_) => len::<u64>(),
			&Type::UnsignedSize(_) | &Type::SignedSize(_) => len::<usize>(),

			&Type::Float32(_) => len::<f32>(),
			&Type::Float64(_) => len::<f64>()
		}
	}
}

pub struct Function{
	name: String,

}
impl Function{
	pub fn call() -> Type{
		Type::Void
	}
}
