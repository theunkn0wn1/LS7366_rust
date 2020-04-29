use crate::errors::DecodeError;
// Anything that can be encoded into a u8
pub trait Encodable {
    fn encode(&self) -> u8;
}
pub trait Decodable: Sized{
    fn decode(raw:u8) -> Result<Self, DecodeError>;
}