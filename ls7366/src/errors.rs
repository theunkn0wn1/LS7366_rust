use bitfield::fmt::Formatter;

#[derive(Clone, Debug)]
pub enum EncoderError {
    FailedDecode,
}

impl core::fmt::Display for EncoderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
