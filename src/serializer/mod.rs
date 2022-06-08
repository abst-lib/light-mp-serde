mod ser_seq;
mod ser_map;
mod ser_struct;
mod ser_variant;

use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use serde::{ser, Serialize, Serializer as SerdeSerializer};
use rmp::encode::{ValueWriteError, self, RmpWriteErr};
use serde::ser::Impossible;
use thiserror::Error;
use crate::serializer::ser_map::{SerializeMap};
use crate::serializer::ser_seq::{SerializeSeq};
use crate::serializer::ser_struct::StructSerializer;
use crate::serializer::ser_variant::VariantSerializer;

#[derive(Debug, Error)]
pub enum SerializeError {
    #[error("Syntax Error {0}")]
    SyntaxError(String),
    #[error("IO Error {0}")]
    InvalidValueWrite(ValueWriteError),
    #[error("Invalid Type {0}")]
    InvalidType(String),

}

impl From<ValueWriteError> for SerializeError {
    fn from(err: ValueWriteError) -> SerializeError {
        SerializeError::InvalidValueWrite(err)
    }
}

impl ser::Error for SerializeError {
    fn custom<T>(msg: T) -> Self where T: Display {
        SerializeError::SyntaxError(msg.to_string())
    }
}

pub struct Serializer<WriteTarget: Write> {
    writer: WriteTarget,
    depth: usize,
}
impl<WriterTarget: Write> Serializer<WriterTarget>{
    pub fn new(writer: WriterTarget) -> Self {
        Serializer {
            writer,
            depth: 0,
        }
    }
}
impl<'writer, WriteTarget: Write> SerdeSerializer for &'writer mut Serializer<WriteTarget> {
    type Ok = ();
    type Error = SerializeError;
    type SerializeSeq = SerializeSeq<'writer, WriteTarget>;
    type SerializeTuple = StructSerializer<'writer, WriteTarget>;
    type SerializeTupleStruct = StructSerializer<'writer, WriteTarget>;
    type SerializeTupleVariant = VariantSerializer<'writer, WriteTarget>;
    type SerializeMap = SerializeMap<'writer, WriteTarget>;
    type SerializeStruct = StructSerializer<'writer, WriteTarget>;
    type SerializeStructVariant = VariantSerializer<'writer, WriteTarget>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        encode::write_bool(&mut self.writer, v)
            .map_err(|err| SerializeError::InvalidValueWrite(ValueWriteError::InvalidMarkerWrite(err)))?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        encode::write_i8(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        encode::write_i16(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        encode::write_i32(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        encode::write_i64(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        encode::write_u8(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        encode::write_u16(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        encode::write_u32(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        encode::write_u64(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        encode::write_f32(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        encode::write_f64(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut tmp = [0u8; 4];
        let value = v.encode_utf8(&mut tmp);
        encode::write_str(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        encode::write_str(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        encode::write_bin(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        encode::write_nil(&mut self.writer)
            .map_err(|err| SerializeError::InvalidValueWrite(ValueWriteError::InvalidMarkerWrite(err)))
    }
    /// Serialize a unit struct as the name
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(name)
    }
    /// Serialize a unit variant as the value of the variant
    fn serialize_unit_variant(self, _: &'static str, _: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        encode::write_map_len(&mut self.writer, 1)?;
        encode::write_str(&mut self.writer, variant)?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Self::SerializeSeq::try_from((self, len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Self::SerializeTuple {
            serializer: self
        })
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Self::SerializeTupleStruct {
            serializer: self
        })
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(Self::SerializeTupleVariant {
            serializer: self
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Self::SerializeMap::try_from((self, len))
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Self::SerializeStruct {
            serializer: self
        })
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(Self::SerializeStructVariant {
            serializer: self
        })
    }
}