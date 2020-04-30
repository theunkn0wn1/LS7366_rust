use crate::errors::EncoderError;

/// Any field that may be encoded into a u8 byte
pub trait Encodable {
    fn encode(&self) -> u8;
}

/// Any object that may be decoded from a u8 byte.
/// Unsuccessful decodes result in an ([`EncoderError`])
///
/// [`EncoderError`]: ../errors/enum.EncoderError.html

pub trait Decodable: Sized{
    fn decode(raw:u8) -> Result<Self, EncoderError>;
}