use embedded_hal::blocking::spi::Transfer;
use embedded_hal::spi::FullDuplex;

use crate::errors::EncoderError;
use crate::ir::Action;
use crate::mdr0::Mdr0;
use crate::traits::Encodable;

pub mod mdr0;
pub mod traits;
pub mod ir;
pub mod errors;
pub mod mdr1;

pub struct Ls7366<SPI> {
    interface: SPI,
}

impl<SPI: FullDuplex<u8> + Transfer<u8>> Ls7366<SPI> {
    pub fn new(iface: SPI) -> Self {
        return Ls7366 {
            interface: iface
        };
    }

    pub fn write_register(mut self, target: ir::Target, payload: &Vec<u8>) -> Result<(), Self::Error> {
        let ir_cmd = ir::InstructionRegister {
            target,
            action: ir::Action::Write,
        };
        if payload.len() > 4 {
            return Err(EncoderError::FailedIO("register payload too big. Expected at most 4 bytes.".to_string()));
        }

        self.interface.send(ir_cmd.encode()).expect_err("failed to transmit");
        for &byte in payload.iter() {
            self.interface.send(byte).expect_err("failed to transmit");
        }
        Ok(())
    }

    pub fn read_register(mut self, target: ir::Target) -> Result<(Vec<u8>), Self::Error> {
        let ir = ir::InstructionRegister {
            target,
            action: Action::Read,
        };
        let mut tx_buffer :Vec<u8> = vec![ir.encode(), 0x00, 0x00, 0x00, 0x00];

        let result = self.interface.transfer(&mut tx_buffer)?;
        Ok(rx_buffer)
    }
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

fn clear_cntr_command() -> Vec<u8> {
    let ir_cmd = ir::InstructionRegister {
        target: ir::Target::Cntr,
        action: ir::Action::Load,
    };
    return vec![ir_cmd.encode()];
}
