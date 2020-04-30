use embedded_hal::blocking::spi::{Transfer, Write};
use nb::block;

use crate::ir::{Action, InstructionRegister};
use crate::mdr0::Mdr0;
use crate::traits::Encodable;

pub mod mdr0;
pub mod traits;
pub mod ir;
pub mod errors;
pub mod mdr1;

#[derive(Clone, Copy, Debug)]
pub enum Error<SpiError> {
    SpiError(SpiError),
    IdMismatch(u8),
    PayloadTooBig,
}

impl<E: std::fmt::Debug> std::error::Error for Error<E> {}

impl<E: std::fmt::Debug> std::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::SpiError(error)
    }
}

pub struct Ls7366<SPI> {
    interface: SPI,
}

impl<SPI, SpiError> Ls7366<SPI>
    where SPI: Transfer<u8, Error=SpiError> + Write<u8, Error=SpiError> {
    pub fn new(iface: SPI) -> Self {
        return Ls7366 {
            interface: iface
        };
    }

    pub fn write_register(&mut self, target: ir::Target, data: &Vec<u8>) -> Result<(), Error<SpiError>> {
        let ir_cmd = ir::InstructionRegister {
            target,
            action: ir::Action::Write,
        };
        if data.len() > 4 {
            return Err((Error::PayloadTooBig));
        }
        let mut payload: Vec<u8> = vec![ir_cmd.encode()];
        payload.extend(data.iter());

        self.interface.write(&payload)?;
        Ok(())
    }

    pub fn read_register(&mut self, target: ir::Target) -> Result<Vec<u8>, Error<SpiError>> {
        let ir = ir::InstructionRegister {
            target,
            action: Action::Read,
        };
        let mut tx_buffer: Vec<u8> = vec![ir.encode(), 0x00, 0x00, 0x00, 0x00];

        let result = self.interface.transfer(&mut tx_buffer)?;
        Ok(Vec::from(result))
    }

    pub fn act(&mut self, command: InstructionRegister, data: Vec<u8>) -> Result<Vec<u8>, Error<SpiError>> {
        let mut tx_buffer: Vec<u8> = vec![command.encode()];
        match command.action {
            Action::Clear | Action::Load => {
                if data.len() > 0 {
                    Err(Error::PayloadTooBig)
                } else {
                    self.interface.write(&tx_buffer)?;
                    Ok(vec![])
                }
            }
            Action::Read => {
                tx_buffer.resize(5, 0x00);
                let mut result = self.interface.transfer(&mut tx_buffer)?;
                Ok(Vec::from(result))
            }
            Action::Write => {
                if data.len() > 4 {
                    Err(Error::PayloadTooBig)
                } else {
                    self.interface.write(&tx_buffer)?;
                    Ok(vec![])
                }
            }
        }
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
