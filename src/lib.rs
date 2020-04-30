//! LS7366 Buffer encoder interface via embedded_hal.
//!
//! # Examples
//! ```no_run
//! use ls7366::Ls7366;
//! use rppal::spi::Spi;
//! fn your_code(spi_interface: Spi){
//!     // Create an instance of the driver from some HAL SPI implementation
//!     let mut driver = Ls7366::new(spi_interface).unwrap();
//!     // Initialize the chip with a sensible default configuration.
//! }
//! ```
//!
use embedded_hal::blocking::spi::{Transfer, Write};

use crate::ir::{Action, InstructionRegister};
use crate::traits::Encodable;

pub mod mdr0;
pub mod traits;
pub mod ir;
pub mod errors;
pub mod mdr1;
mod utilities;

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

/// An LS8366 Quadrature encoder buffer
pub struct Ls7366<SPI> {
    /// SPI interface where the buffer is attached.
    interface: SPI,
}

impl<SPI, SpiError> Ls7366<SPI>
    where SPI: Transfer<u8, Error=SpiError> + Write<u8, Error=SpiError> {
    /// Creates a new driver and initializes the Chip to some sensible default values.
    /// This will zero the chip's counter, configure it to 4 byte count mode (full range)
    /// and to treat every 4th quadrature pulse as a increment.
    ///
    /// If another configuration is desirable, see [`mdr0.Mdr0`] and [`mdr1.Mdr1`]
    /// for all available options.
    pub fn new(iface: SPI) -> Result<Self, Error<SpiError>> {
        let mut driver = Ls7366 {
            interface: iface
        };
        // Creating configurations for the two MDR configuration registers
        let mdr0_payload = mdr0::Mdr0 {
            quad_count_mode: mdr0::QuadCountMode::Quad4x,
            cycle_count_mode: mdr0::CycleCountMode::FreeRunning,
            index_mode: mdr0::IndexMode::DisableIndex,
            is_index_inverted: false,
            filter_clock: mdr0::FilterClockDivisionFactor::One,
        };
        let mdr1_payload = mdr1::Mdr1 {
            counter_mode: mdr1::CounterMode::Byte4,
            enable_counting: true,
            flag_on_idx: false,
            flag_on_cmp: false,
            flag_on_bw: false,
            flag_on_cy: false,
        };

        // Write primary configuration to chip.
        driver.write_register(ir::Target::Mdr0, &vec![mdr0_payload.encode()])?;
        // Write secondary configuration to chip.
        driver.write_register(ir::Target::Mdr1, &vec![mdr1_payload.encode()])?;
        // Zero Dtr to prepare a write into Cntr.
        driver.write_register(ir::Target::Dtr, &vec![0x00, 0x00, 0x00, 0x00])?;
        // Load Dtr into Cntr.
        driver.act(
            ir::InstructionRegister {
                target: ir::Target::Cntr,
                action: ir::Action::Load,
            },
            vec![],
        )?;
        Ok(driver)
    }

    pub fn write_register(&mut self, target: ir::Target, data: &Vec<u8>) -> Result<(), Error<SpiError>> {
        let ir_cmd = ir::InstructionRegister {
            target,
            action: ir::Action::Write,
        };
        if data.len() > 4 {
            return Err(Error::PayloadTooBig);
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


    pub fn get_count(&mut self) -> Result<u32, Error<SpiError>> {
        let raw_result = self.read_register(ir::Target::Cntr)?;
        Ok(utilities::vec_to_u32(&raw_result))
    }


    /// Performs a transaction against the chip.
    ///
    /// Some actions (e.g. writing to a register) accept up to 4 u8 bytes, this function accepts
    /// the same. Attempt to write more than 4 bytes will result in an ['SpiError.PayloadToBig`]
    ///
    /// Other sources of error responses may arise from the underlying HAL implementation and are
    /// bubbled up.
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
                let result = self.interface.transfer(&mut tx_buffer)?;
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
