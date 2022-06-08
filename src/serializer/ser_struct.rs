use std::io::Write;
use serde::ser::{SerializeStruct, SerializeTuple, SerializeTupleStruct};
use serde::Serialize;

pub struct StructSerializer<'writer, Writer: Write> {
    pub(crate) serializer: &'writer mut super::Serializer<Writer>,
}

impl<'writer, Writer: Write> SerializeTupleStruct for StructSerializer<'writer, Writer> {
    type Ok = ();
    type Error = super::SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        value.serialize(&mut *self.serializer)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}


// light-mp-serde is all based on positional serialization. This means that the order of the fields in the struct is important. But names are ignored
impl<'writer, Writer: Write> SerializeStruct for StructSerializer<'writer, Writer> {
    type Ok = <Self as SerializeTupleStruct>::Ok;
    type Error = <Self as SerializeTupleStruct>::Error;

    fn serialize_field<T: ?Sized>(&mut self, _: &'static str, value: &T) -> Result<(), Self::Error> where T: Serialize {
        <Self as SerializeTupleStruct>::serialize_field(self, value)
    }


    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as SerializeTupleStruct>::end(self)
    }
}

impl<'writer, Writer: Write> SerializeTuple for StructSerializer<'writer, Writer> {
    type Ok = <Self as SerializeTupleStruct>::Ok;
    type Error = <Self as SerializeTupleStruct>::Error;


    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as SerializeTupleStruct>::end(self)
    }

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        <Self as SerializeTupleStruct>::serialize_field(self, value)
    }
}