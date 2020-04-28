
// Anything that can be encoded into a u8
pub trait Encodable {
    fn encode(&self) -> u8;
}