#![feature(slice_from_ptr_range)]
#![feature(portable_simd)]
#![feature(core_intrinsics)]

//! Serializer and deserializer implementation for
//! The Elder Scrolls Online encounter log format
//! 
//! You probably don't want to use this, and use `eso_lib` instead

mod eso_reader;
pub mod eso_serde;

pub use eso_reader::*;
mod number_parser;
