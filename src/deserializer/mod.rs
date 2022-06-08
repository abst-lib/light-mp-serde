use std::fmt::Display;
use std::io::Read;
use rmp::decode::ValueReadError;
use serde::de::{Deserializer as SerdeDeserializer, Visitor};
use thiserror::Error;
#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error("Syntax Error {0}")]
    SyntaxError(String),
    #[error("IO Error {0}")]
    InvalidValueRead(ValueReadError),
    #[error("Invalid Type {0}")]
    InvalidType(String),

}

impl From<ValueReadError> for DeserializeError {
    fn from(err: ValueReadError) -> DeserializeError {
        DeserializeError::InvalidValueRead(err)
    }
}

impl serde::de::Error for DeserializeError {
    fn custom<T>(msg: T) -> Self where T: Display {
        DeserializeError::SyntaxError(msg.to_string())
    }
}

pub struct Deserializer<R: Read> {
    reader: R,
    depth: usize,
}

impl<R: Read> Deserializer<R> {
    pub fn new(read: R) -> Self {
        Deserializer {
            reader: read,
            depth: 0,
        }
    }
}
