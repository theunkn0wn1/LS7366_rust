use std::error::Error;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use ls7366::Ls7366;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    // create an instance of an SPI object
    // In this case, the buffer is on SPI0 and SS1.
    // The chip acts in Mode0.
    let spi_1 = Spi::new(Bus::Spi0, SlaveSelect::Ss1, 14_000_000, Mode::Mode0)?;

    // Construct a driver instance from the SPI interface, using default chip configurations.
    let mut spi_driver = Ls7366::new(spi_1)?;

    // Loop and read the counter.
    loop {
        let result = spi_driver.get_count()?;
        sleep(Duration::from_secs(1));
        println!("read data:= {:?}", result);
    }
}