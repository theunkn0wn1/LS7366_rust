use embedded_hal::blocking::spi::{Write, Transfer};

struct SomeSpi{

}

impl Write<u8> for SomeSpi {
    type Error = ();

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }
}
impl Transfer<u8> for SomeSpi{
    type Error = ();

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        unimplemented!()
    }
}