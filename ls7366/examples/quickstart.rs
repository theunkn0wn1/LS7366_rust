use std::thread::sleep;
use std::time::Duration;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use ls7366::Ls7366;

fn main() {
    // create an instance of an SPI object
    // In this case, the buffer is on SPI0 and SS1.
    // The chip acts in Mode0.
    let spi_1 = Spi::new(Bus::Spi0, SlaveSelect::Ss1, 14_000_000, Mode::Mode0).unwrap();
    // Construct a driver instance from the SPI interface, using default chip configurations.
    let mut spi_driver = Ls7366::new(spi_1).unwrap();

    // Loop and read the counter.
    loop {
        let result = spi_driver.get_count().unwrap();
        let status = spi_driver.get_status().unwrap();
        println!("read data:= {:?}\n status := {:?}", result, status);
        sleep(Duration::from_secs(1));
    }

}
