
pub mod deserializer;
mod serializer;
mod error;
pub mod newtypes;

pub use serializer::*;
pub use error::*;

pub type Deserializer<'de> = deserializer::Deserializer<'de, super::eso_reader::EsoLogReader>;

/// # Warning
/// This deserializer is `unsafe to use`
/// please ensure that passed string is safe to read
/// atleast 32 bytes past its end
/// 
/// Not holding this guarantee will cause `UB`, and most likely a `SEGFAULT` in best case
pub type UnguardedDeserializer<'de> = deserializer::Deserializer<'de, super::eso_reader::UnguardedEsoLogReader>;
