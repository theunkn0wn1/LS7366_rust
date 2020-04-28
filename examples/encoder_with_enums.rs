use std::error::Error;

use rppal::spi::{Bus, Mode, Result as SpiResult, SlaveSelect, Spi};
use rppal::system::DeviceInfo;

use ls7366::{RawMessage, RegisterAction, RegisterSelection};
use ls7366::traits::Encodable;

fn main() -> Result<(), Box<dyn Error>> {
    let device = DeviceInfo::new()?.model();
    println!("device model := {}", device);
    let spi_0 = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 14000000, Mode::Mode0)?;
    let spi_1 = Spi::new(Bus::Spi0, SlaveSelect::Ss1, 14000000, Mode::Mode0)?;

    let mut action = RawMessage(0x00);
    action.set_register(RegisterSelection::SelectMdr0.encode());
    action.set_action(RegisterAction::WriteRegister.encode());
    Ok(())
}