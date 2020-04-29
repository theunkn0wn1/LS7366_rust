use std::error::Error;
use bitfield::fmt::Formatter;

#[derive(Debug)]
pub enum DecodeError {
    Failed(String)
}
impl Error for DecodeError{

}
impl std::fmt::Display for DecodeError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}