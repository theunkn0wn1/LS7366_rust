use std::error::Error;
use bitfield::fmt::Formatter;

#[derive(Debug)]
pub enum EncoderError {
    FailedDecode(String),
    FailedIO(String),
}
impl Error for EncoderError {

}
impl std::fmt::Display for EncoderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}