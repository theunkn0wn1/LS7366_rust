use std::error::Error;

use rppal::spi::{Bus, Mode, Result as SpiResult, SlaveSelect, Spi};
use rppal::system::DeviceInfo;

use ls7366::{RawMessage, RegisterAction, RegisterSelection};
use ls7366::mdr0;
use ls7366::traits::Encodable;

fn main() -> Result<(), Box<dyn Error>> {
//    let device = DeviceInfo::new()?.model();
//    println!("device model := {}", device);
//    let spi_0 = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 14000000, Mode::Mode0)?;
//    let spi_1 = Spi::new(Bus::Spi0, SlaveSelect::Ss1, 14000000, Mode::Mode0)?;

    let mdr0_payload = mdr0::Mdr0{
        quad_count_mode: mdr0::QuadCountMode::NonQuad,
        cycle_count_mode: mdr0::CycleCountMode::FreeRunning,
        index_mode: mdr0::IndexMode::DisableIndex,
        is_index_inverted: false,
        filter_clock: mdr0::FilterClockDivisionFactor::One
    };

    let encoded = mdr0_payload.encode();
    println!("mdr0 := {:?}", encoded);
    Ok(())
}