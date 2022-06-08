use std::io::Write;
use serde::ser::{SerializeStructVariant, SerializeTuple, SerializeTupleVariant};
use serde::Serialize;

pub struct VariantSerializer<'writer, Writer: Write> {
    pub(crate) serializer: &'writer mut super::Serializer<Writer>,
}

impl<'writer, Writer: Write> SerializeTupleVariant for VariantSerializer<'writer, Writer> {
    type Ok = ();
    type Error =super::SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'writer, Writer: Write> SerializeStructVariant for VariantSerializer<'writer, Writer> {
    type Ok = <Self as SerializeTupleVariant>::Ok;
    type Error = <Self as SerializeTupleVariant>::Error;

    fn serialize_field<T: ?Sized>(&mut self, _: &'static str, value: &T) -> Result<(), Self::Error> where T: Serialize {
        <Self as SerializeTupleVariant>::serialize_field(self, value)
    }


    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as SerializeTupleVariant>::end(self)
    }
}
