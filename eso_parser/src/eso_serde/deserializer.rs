
use std::marker::PhantomData;

use serde::de::{self, SeqAccess, EnumAccess, VariantAccess};

use crate::{EsoLogLineReaderTrait, EsoLogReaderTrait, EsoReaderTrait, number_parser::NumberParser};
use super::error::Error;

/// The Elder Scrolls Online ocmpatible encounter log format deserializer
///
/// Must be used in conjunction with type implementing `EsoReaderTrait`,
/// eg. `EsoLogReader` or `UnguardedEsoLogReader`
pub struct Deserializer<'de, Reader: EsoReaderTrait<'de>> {
    s: Reader::LineReader,
    _phantom: PhantomData<&'de ()>,
}

impl<'de, Reader: EsoReaderTrait<'de>> Deserializer<'de, Reader> {
    pub fn new(s: &'de str) -> Self {
        Self {
            s: Reader::read_line(s),
            _phantom: Default::default(),
        }
    }

    #[inline]
    pub fn is_depleted(&self) -> bool {
        self.s.is_depleted()
    }

    #[inline]
    fn next(&mut self) -> Result<&'de str, Error> {
        if let Some(value) = Iterator::next(&mut self.s) {
            Ok(value)
        } else {
            Err(Error::UnexpectedEnd)
        }
    }

    #[inline]
    fn read_number<T: NumberParser>(&mut self) -> Result<T, Error> {
        let (number, parsed_bytes) = T::parse(self.s.inner().as_bytes())?;
        unsafe { self.s.advance(parsed_bytes) };

        Ok(number)
    }
}

struct VecWrapper<'de, Reader: EsoReaderTrait<'de>>(Deserializer<'de, Reader>);

impl<'de, Reader: EsoReaderTrait<'de>> VecWrapper<'de, Reader> {
    fn new(s: &'de str) -> Result<Self, Error> {
        let slice = s.as_bytes();

        if slice.first() == Some(&b'[')
        && slice.last() == Some(&b']') {
            Ok(Self(Deserializer {
                s: Reader::read_vec(s),
                _phantom: Default::default(),
            }))
        } else {
            Err(Error::NotAList)
        }
    }
}

struct SeqWrapper<'de, 'a, Reader: EsoReaderTrait<'de>>(&'a mut Deserializer<'de, Reader>);

impl<'de, 'a, Reader: EsoReaderTrait<'de>> SeqWrapper<'de, 'a, Reader> {
    fn new(de: &'a mut Deserializer<'de, Reader>, _len: usize) -> Self {
        Self(de)
    }
}

struct Enum<'de, 'a, Reader: EsoReaderTrait<'de>>(&'a mut Deserializer<'de, Reader>);

impl<'de, 'a, Reader: EsoReaderTrait<'de>> Enum<'de, 'a, Reader> {
    fn new(de: &'a mut Deserializer<'de, Reader>) -> Self {
        Self(de)
    }
}

impl<'de, 'a, Reader: EsoReaderTrait<'de>> de::Deserializer<'de> for &'a mut Deserializer<'de, Reader> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let result = match self.next()? {
            "T" => true,
            "F" => false,
            _ => {
                // println!("{other}");
                return Err(Error::InvalidToken);
            },
        };

        visitor.visit_bool(result)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.read_number()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.read_number()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.read_number()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.read_number()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.read_number()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.read_number()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.read_number()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.read_number()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.read_number()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(self.read_number()?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let string = self.next()?;
        
        visitor.visit_borrowed_str(string)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()    
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.s.is_depleted() {
            visitor.visit_none()
        } else {
            let mut copy = Deserializer {
                s: self.s.clone(),
                _phantom: Default::default(),
            };

            match copy.next()? {
                "*" => {
                    *self = copy;
    
                    visitor.visit_none()
                },
                _ => {
                    visitor.visit_some(self)
                }
            }
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(VecWrapper::<Reader>::new(self.next()?)?)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SeqWrapper::new(self, len))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(VecWrapper::<Reader>::new(self.next()?)?)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SeqWrapper::new(self, fields.len()))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(Enum::new(self))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedOperation("Map deserialization is not supported by this deserializer"))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }
}

impl<'de, Reader: EsoReaderTrait<'de>> SeqAccess<'de> for VecWrapper<'de, Reader> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if !self.0.is_depleted() {
            seed.deserialize(&mut self.0).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl<'de, 'a, Reader: EsoReaderTrait<'de>> SeqAccess<'de> for SeqWrapper<'de, 'a, Reader> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.0).map(Some)
    }
}

impl<'de, 'a, Reader: EsoReaderTrait<'de>> EnumAccess<'de> for Enum<'de, 'a, Reader> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.0)?;

        Ok((val, self))
    }
}

impl<'de, 'a, Reader: EsoReaderTrait<'de>> VariantAccess<'de> for Enum<'de, 'a, Reader> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.0)
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(&mut *self.0, "", fields, visitor)
    }
}
