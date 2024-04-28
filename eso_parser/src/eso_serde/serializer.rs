use super::SerializeError;
use serde::{ser, Serialize};

pub struct Serializer {
    out: String,
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            out: String::with_capacity(128)
        }
    }

    pub fn as_str(&self) -> &str {
        let mut chars = self.out.chars();
        chars.next_back();

        chars.as_str()
    }

    pub fn into_string(mut self) -> String {
        self.out.pop();

        self.out
    }

    fn push_str(&mut self, s: &str) -> Result<(), SerializeError> {
        self.out.push_str(s);
        self.out.push(',');

        Ok(())
    }

    #[inline]
    fn serialize_unsigned(&mut self, mut i: u64) -> Result<(), SerializeError> {
        if i == 0 {
            self.out.push_str("0,");
            return Ok(());
        }

        let buf = unsafe { self.out.as_mut_vec() };
        let start = buf.len();

        while i != 0 {
            let rem = (i % 10) as u8;
            i /= 10;

            buf.push(b'0' + rem);
        }

        unsafe {
            buf.split_at_mut_unchecked(start).1.reverse();
        }

        buf.push(b',');
        Ok(())
    }

    #[inline]
    fn serialize_signed(&mut self, mut i: i64) -> Result<(), SerializeError> {
        if i == 0 {
            self.out.push_str("0,");
            return Ok(());
        }

        if i < 0 {
            self.out.push('-');
        }

        let buf = unsafe { self.out.as_mut_vec() };
        let start = buf.len();
        buf.reserve(20);

        while i != 0 {
            let rem = (i % 10) as u8;
            i /= 10;

            buf.push(b'0' + rem);
        }

        unsafe {
            buf.split_at_mut_unchecked(start).1.reverse();
        }

        buf.push(b',');
        Ok(())
    }
}

impl<'a> serde::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    type SerializeMap = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let v = if v { "T" } else { "F" };
        self.push_str(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_signed(v as _)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_signed(v as _)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_signed(v as _)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_signed(v as _)
    }
    
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_unsigned(v as _)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_unsigned(v as _)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_unsigned(v as _)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_unsigned(v as _)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        
        self.push_str(&v.to_string())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        
        self.push_str(&v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0; 4];

        self.push_str(v.encode_utf8(&mut buf))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.push_str(v)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::UnsupportedOperation("this serializer doesn't support serializing bytes"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error>
    {
        self.push_str(variant)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        variant.serialize(&mut *self)?;
        value.serialize(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.out.push('[');

        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error>
    {
        self.out.push('[');

        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error>
    {
        todo!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>
    {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error>
    {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.out.push('[');

        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(SerializeError::UnsupportedOperation(MAP_SERIALIZATION_NOT_SUPPORTED))
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.out.as_bytes().last() == Some(&b',') {
            unsafe {
                *self.out
                    .as_bytes_mut()
                    .last_mut()
                    .unwrap_unchecked() = b']';
            }
        } else {
            self.out.push(']');
        }

        self.out.push(',');
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

const MAP_SERIALIZATION_NOT_SUPPORTED: &'static str = "Map serialization is not supported by this serializer";

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        _key: &K,
        _value: &V,
    ) -> Result<(), Self::Error>
    where
        K: serde::Serialize,
        V: serde::Serialize,
    {
        Err(SerializeError::UnsupportedOperation(MAP_SERIALIZATION_NOT_SUPPORTED))
    }

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        Err(SerializeError::UnsupportedOperation(MAP_SERIALIZATION_NOT_SUPPORTED))
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        Err(SerializeError::UnsupportedOperation(MAP_SERIALIZATION_NOT_SUPPORTED))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::UnsupportedOperation(MAP_SERIALIZATION_NOT_SUPPORTED))
    }
}
