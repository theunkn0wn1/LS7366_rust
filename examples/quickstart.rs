use std::error::Error;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use ls7366::Ls7366;

fn main() -> Result<(), Box<dyn Error>> {
//    let spi_0 = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 14000000, Mode::Mode0)?;
    // create an instance of an SPI object
    // In this case, the buffer is on SPI0 and SS1.
    // The chip acts in Mode0.
    let spi_1 = Spi::new(Bus::Spi0, SlaveSelect::Ss1, 14_000_000, Mode::Mode0)?;

    // Hand the SPI interface over to the driver.
    let mut spi_driver = Ls7366::new(spi_1)?;


    // Loop and read the counter.
    loop {
        let result = spi_driver.get_count()?;
        println!("read data:= {:?}", result);
    }
}