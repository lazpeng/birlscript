//! Defines stuff that are arch-specific
//!
#[cfg(target_pointer_width = "64")]
pub type IntegerType = i64;

#[cfg(target_pointer_width = "32")]
pub type IntegerType = i32;