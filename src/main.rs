use std::error::Error;

use rppal::spi::{Bus, Mode, Result as SpiResult, SlaveSelect, Spi};
use rppal::system::DeviceInfo;

const WRITE_MDR0: u8 = 0x88;
const WRITE_DTR: u8 = 0x98;
const CONFIGURE4BYTE_MODE: u8 = 0x03;
const REQUEST_COUNT: u8 = 0x60;
const SET_ENCODER_TO_CENTER: u8 = 0xE0;

const INIT_PAYLOAD: [u8; 4] = [WRITE_MDR0, CONFIGURE4BYTE_MODE, 0x00, 0x00];
const ZERO_PAYLOAD: [u8; 5] = [WRITE_DTR, 0x00, 0x00, 0x00, 0x00];
const REQUEST_PAYLOAD: [u8; 5] = [REQUEST_COUNT, 0x00, 0x00, 0x00, 0x00];
const CENTER_PAYLOAD: [u8; 1] = [SET_ENCODER_TO_CENTER];

fn main() -> Result<(), Box<dyn Error>> {
    let device = DeviceInfo::new()?.model();
    println!("device model := {}", device);
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 14000000, Mode::Mode0)?;

    init_encoder(&spi)?;
    zero_encoder(&spi)?;
    center_encoder(&spi)?;
    loop {
        println!("read: {:?}", read_encoder(&spi)?);
    }
}

fn init_encoder(spi: &Spi) -> SpiResult<usize> {
    let mut recv_buffer: [u8; INIT_PAYLOAD.len()] = [0; INIT_PAYLOAD.len()];
    spi.transfer(&mut recv_buffer, &INIT_PAYLOAD)
}

fn zero_encoder(spi: &Spi) -> SpiResult<usize> {
    let mut recv_buffer: [u8; ZERO_PAYLOAD.len()] = [0; ZERO_PAYLOAD.len()];
    spi.transfer(&mut recv_buffer, &ZERO_PAYLOAD)
}

fn read_encoder(spi: &Spi) -> SpiResult<[u8; REQUEST_PAYLOAD.len()]> {
    let mut recv_buffer: [u8; REQUEST_PAYLOAD.len()] = [0x00; REQUEST_PAYLOAD.len()];
    spi.transfer(&mut recv_buffer, &REQUEST_PAYLOAD)?;
    Ok(recv_buffer)
}

fn center_encoder(spi: &Spi) -> SpiResult<usize> {
    let mut recv_buffer: [u8; CENTER_PAYLOAD.len()] = [0; CENTER_PAYLOAD.len()];
    spi.transfer(&mut recv_buffer, &CENTER_PAYLOAD)
}
