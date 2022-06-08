use std::io::Write;
use rmp::encode;
use rmp::encode::{ValueWriteError, write_array_len};
use serde::ser::{SerializeSeq as SerdeSerializeSeq};
use serde::Serialize;
use crate::serializer::Serializer;

pub struct KnownLengthSeqSerializer<'writer, Writer: Write> {
    serializer: &'writer mut  super::Serializer<Writer>,
}

impl<'writer, Writer: Write + 'writer> SerdeSerializeSeq for KnownLengthSeqSerializer<'writer, Writer> {
    type Ok = ();
    type Error = super::SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        value.serialize(&mut *self.serializer)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct UnKnownLengthSeqSerializer<'writer, Writer: Write> {
    parent_serializer: &'writer mut super::Serializer<Writer>,
    vec_holder: super::Serializer<Vec<u8>>,
    length: u32,
}

impl<'writer, Writer: Write + 'writer> SerdeSerializeSeq for UnKnownLengthSeqSerializer<'writer, Writer> {
    type Ok = ();
    type Error = super::SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        value.serialize(&mut self.vec_holder)?;
        self.length += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        encode::write_array_len(&mut self.parent_serializer.writer, self.length)?;
        self.parent_serializer.writer.write_all(&self.vec_holder.writer).map_err(|err| super::SerializeError::InvalidValueWrite(ValueWriteError::InvalidDataWrite(err)))?;
        Ok(())
    }
}

pub enum SerializeSeq<'writer, Writer: Write> {
    KnownLength(KnownLengthSeqSerializer<'writer, Writer>),
    UnKnownLength(UnKnownLengthSeqSerializer<'writer, Writer>),
}

impl<'writer, Writer: Write> SerdeSerializeSeq for SerializeSeq<'writer, Writer> {
    type Ok = ();
    type Error = super::SerializeError;


    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        match self {
            SerializeSeq::KnownLength(seq) => seq.serialize_element(value),
            SerializeSeq::UnKnownLength(seq) => seq.serialize_element(value),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            SerializeSeq::KnownLength(seq) => seq.end(),
            SerializeSeq::UnKnownLength(seq) => seq.end(),
        }
    }
}
impl<'writer, Writer: Write> TryFrom<(&'writer mut super::Serializer<Writer>, Option<usize>)> for SerializeSeq<'writer, Writer>{
    type Error = super::SerializeError;

    fn try_from((serializer, length): (&'writer mut Serializer<Writer>, Option<usize>)) -> Result<Self, Self::Error> {

        match length {
            Some(length) => {
                write_array_len(&mut serializer.writer, length as u32)?;
                let seq = SerializeSeq::KnownLength(KnownLengthSeqSerializer {
                    serializer,
                });
                Ok(seq)
            },
            None => Ok(SerializeSeq::UnKnownLength(UnKnownLengthSeqSerializer {
                parent_serializer: serializer,
                vec_holder: super::Serializer{
                    writer: Vec::new(),
                    depth: 0
                },
                length: 0,
            })),
        }
    }
}