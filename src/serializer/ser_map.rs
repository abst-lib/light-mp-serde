use std::io::Write;
use rmp::encode;
use rmp::encode::{ValueWriteError, write_array_len};
use serde::ser::SerializeMap as SerdeSerializeMap;
use serde::Serialize;
use crate::serializer::Serializer;

pub struct KnownLengthMapSerializer<'writer, Writer: Write> {
    serializer: &'writer mut super::Serializer<Writer>,
}

impl<'writer, Writer: Write + 'writer> SerdeSerializeMap for KnownLengthMapSerializer<'writer, Writer> {
    type Ok = ();
    type Error = super::SerializeError;


    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> where T: Serialize {
        key.serialize(&mut *self.serializer)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        value.serialize(&mut *self.serializer)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct UnKnownLengthMapSerializer<'writer, Writer: Write> {
    parent_serializer: &'writer mut super::Serializer<Writer>,
    map_holder: super::Serializer<Vec<u8>>,
    length: u32,
}

impl<'writer, Writer: Write + 'writer> SerdeSerializeMap for UnKnownLengthMapSerializer<'writer, Writer> {
    type Ok = ();
    type Error = super::SerializeError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> where T: Serialize {
        key.serialize(&mut self.map_holder)?;
        self.length += 1;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        value.serialize(&mut self.map_holder)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        encode::write_map_len(&mut self.parent_serializer.writer, self.length)?;
        self.parent_serializer.writer.write_all(&self.map_holder.writer).map_err(|err| super::SerializeError::InvalidValueWrite(ValueWriteError::InvalidDataWrite(err)))?;
        Ok(())
    }
}

pub enum SerializeMap<'writer, Writer: Write> {
    KnownLength(KnownLengthMapSerializer<'writer, Writer>),
    UnKnownLength(UnKnownLengthMapSerializer<'writer, Writer>),
}

impl<'writer, Writer: Write> SerdeSerializeMap for SerializeMap<'writer, Writer> {
    type Ok = ();
    type Error = super::SerializeError;


    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> where T: Serialize {
        match self {
            SerializeMap::KnownLength(ref mut seq) => seq.serialize_key(key),
            SerializeMap::UnKnownLength(ref mut seq) => seq.serialize_key(key),
        }
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        match self {
            SerializeMap::KnownLength(ref mut seq) => seq.serialize_value(value),
            SerializeMap::UnKnownLength(ref mut seq) => seq.serialize_value(value),
        }
    }


    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            SerializeMap::KnownLength(seq) => seq.end(),
            SerializeMap::UnKnownLength(seq) => seq.end(),
        }
    }
}

impl<'writer, Writer: Write> TryFrom<(&'writer mut super::Serializer<Writer>, Option<usize>)> for SerializeMap<'writer, Writer> {
    type Error = super::SerializeError;

    fn try_from((serializer, length): (&'writer mut Serializer<Writer>, Option<usize>)) -> Result<Self, Self::Error> {
        match length {
            Some(length) => {
                encode::write_map_len(&mut serializer.writer, length as u32)?;
                let seq = SerializeMap::KnownLength(KnownLengthMapSerializer {
                    serializer,
                });
                Ok(seq)
            }
            None => Ok(SerializeMap::UnKnownLength(UnKnownLengthMapSerializer {
                parent_serializer: serializer,
                map_holder: super::Serializer {
                    writer: Vec::new(),
                    depth: 0,
                },
                length: 0,
            })),
        }
    }
}