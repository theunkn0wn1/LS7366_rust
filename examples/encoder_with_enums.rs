use std::error::Error;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::system::DeviceInfo;

use ls7366::ir;
use ls7366::mdr0;
use ls7366::traits::{Decodable, Encodable};

fn main() -> Result<(), Box<dyn Error>> {
    let device = DeviceInfo::new()?.model();
    println!("device model := {}", device);
    let spi_0 = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 14000000, Mode::Mode0)?;
    let spi_1 = Spi::new(Bus::Spi0, SlaveSelect::Ss1, 14000000, Mode::Mode0)?;

    let mdr0_payload = mdr0::Mdr0 {
        quad_count_mode: mdr0::QuadCountMode::Quad4x,
        cycle_count_mode: mdr0::CycleCountMode::FreeRunning,
        index_mode: mdr0::IndexMode::ClearCntr,
        is_index_inverted: false,
        filter_clock: mdr0::FilterClockDivisionFactor::One,
    };
    let mut read_buffer: Vec<u8> = vec![];
    spi_1.transfer(&mut read_buffer, &init_command(mdr0_payload))?;
    println!("initialized spi1, result := {:?}", read_buffer);
    read_buffer.clear();
    spi_1.transfer(&mut read_buffer, &zero_dtr_command())?;
    println!("zero'ed spi1's DTR, result := {:?}", read_buffer);
    read_buffer.clear();

    spi_1.transfer(&mut read_buffer, &transfer_dtr_to_cntr_command())?;
    println!("zero'ed spi1's CNTR, result := {:?}", read_buffer);
    loop {
        read_buffer.clear();
        read_buffer.resize(5, 0x00);

        // last but NOT least, try to read the counter!
        spi_1.transfer(&mut read_buffer, &read_cntr_command())?;
        println!("read from SPI1, value := {:?}", read_buffer);
    }
    Ok(())
}

fn init_command(configuration: mdr0::Mdr0) -> Vec<u8> {
    let ir_cmd = ir::InstructionRegister {
        target: ir::Target::Mdr0,
        action: ir::Action::Write,
    };

    return vec![ir_cmd.encode(), configuration.encode()];
}

fn zero_dtr_command() -> Vec<u8> {
    let ir_cmd = ir::InstructionRegister {
        target: ir::Target::Dtr,
        action: ir::Action::Write,
    };
    return vec![ir_cmd.encode(), 0x00, 0x00, 0x00, 0x00];
}

fn transfer_dtr_to_cntr_command() -> Vec<u8> {
    let irc_cmd = ir::InstructionRegister {
        target: ir::Target::Cntr,
        action: ir::Action::Load,
    };
    return vec![irc_cmd.encode()];
}

fn read_cntr_command() -> Vec<u8> {
    let ir_cmd = ir::InstructionRegister {
        target: ir::Target::Cntr,
        action: ir::Action::Read,
    };
    return vec![ir_cmd.encode(), 0x00, 0x00, 0x00, 0x00];
}