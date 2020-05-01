//! LS7366 Buffer encoder interface using `embedded_hal`.
//!
//! This driver should work with any SPI interface as long as it implements
//! the blocking `embedded_hal` [`SPI traits`].
//!
//!
//! # Examples
//! Bare-minimum boilerplate to read from the buffer:
//! ```no_run
//!   use ls7366::Ls7366;
//! // --- snip ---
//! # use std::error::Error;
//! #
//! # use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
//! #
//! # use std::thread::sleep;
//! # use std::time::Duration;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! #    // create an instance of an SPI object
//! #    // In this case, the buffer is on SPI0 and SS1.
//! #    // The chip acts in Mode0.
//! #    let some_hal_spi_object = Spi::new(Bus::Spi0, SlaveSelect::Ss1, 14_000_000, Mode::Mode0)?;
//! #
//!     // Construct a driver instance from the SPI interface, using default chip configurations.
//!     let mut spi_driver = Ls7366::new(some_hal_spi_object)?;
//!
//!     // Loop and read the counter.
//!     loop {
//!         let result = spi_driver.get_count()?;
//!         sleep(Duration::from_secs(1));
//!         println!("read data:= {:?}", result);
//!     }
//! // --- snip ---
//! # }
//! ```
//! ## Advanced configuration
//! The LS7366 has two registers dedicated to configuring the chip's various functions:
//! [`Mdr0`] and [`Mdr1`].
//!
//! Configuring the chip can be accomplished by writing into these two registers.
//!
//! **Manually configuring these registers is **not** required when using [`Ls7366::new`].**
//!
//! 1. Build an instance of [`Mdr0`] and [`Mdr1`] with the desired configuration.
//! 2. Write these instances into the relevant registers.
//! ```no_run
//! use ls7366::mdr0::{QuadCountMode, CycleCountMode, FilterClockDivisionFactor,IndexMode, Mdr0};
//! use ls7366::mdr1::{CounterMode, Mdr1};
//! use ls7366::{Ls7366, Target, Encodable};
//! # use rppal::spi::Spi; // concrete SPI implementation
//! # use std::error::Error;
//! // --- snip ---
//! # fn your_code(spi_driver: &mut Ls7366<Spi>) -> Result<(), Box<dyn Error>> {
//!     let mdr0_configuration = Mdr0{
//!         quad_count_mode: QuadCountMode::Quad2x,
//!         filter_clock : FilterClockDivisionFactor::Two,
//!         index_mode: IndexMode::ClearCntr,
//!         cycle_count_mode: CycleCountMode::SingleCycle,
//!         is_index_inverted: false
//!     };
//!     let mdr1_configuration = Mdr1{
//!         counter_mode: CounterMode::Byte3,
//!         // --- Snip ---
//!         # disable_counting:true,
//!         # flag_on_bw: false,
//!         # flag_on_idx: false,
//!         # flag_on_cmp: false,
//!         # flag_on_cy: false,
//!     };
//!
//!     spi_driver.write_register(Target::Mdr0, &vec![mdr0_configuration.encode()])?;
//!     spi_driver.write_register(Target::Mdr1, &vec![mdr1_configuration.encode()])?;
//!     // --- Snip ---
//!     # Ok(())
//! }
//! ```
//!
//! [`SPI traits`]: ../embedded_hal/blocking/spi/index.html
//! [`Mdr0`]: ./mdr0/struct.Mdr0.html
//! [`Mdr1`]: ./mdr1/struct.Mdr1.html
//! [`Ls7366::new`]: ./struct.Ls7366.html#method.new

use embedded_hal::blocking::spi::{Transfer, Write};

pub use crate::ir::{Action, Target};
use crate::ir::InstructionRegister;
use crate::str_register::Str;
use crate::traits::Decodable;
pub use crate::traits::Encodable;

pub mod mdr0;
pub mod ir;
pub mod mdr1;
pub mod str_register;
mod traits;
mod errors;
mod utilities;
mod test_instruction_register;
mod test_io;

#[derive(Clone, Debug)]
pub enum Error<SpiError> {
    // Underlying SPI interface error
    SpiError(SpiError),
    // Failed to encode / decode payload
    EncodeError(errors::EncoderError),
    // Request to write payload larger than target register.
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
    /// If the chip is already configured or another configuration is preferable,
    /// use the ([`uninit`]) constructor.
    ///
    /// [`uninit`]: #method.new_uninit
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
            disable_counting: false,
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

    /// Creates a new driver but does NOT do any initialization actions against the chip.
    pub fn new_uninit(iface: SPI) -> Self {
        Ls7366 {
            interface: iface
        }
    }
    /// Writes bytes into the specified register. attempting to write more than 4 bytes is an error.
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
    /// Executes a read operation against specified register, returning up to 4 bytes from the chip.
    ///
    /// ## Note:
    ///
    /// Reading from [`Str`] clears the register to zero.
    ///
    /// Reading from [`Dtr`] is a Noop.
    ///
    /// Reading from [`Cntr`] overwrites [`Otr`].
    ///
    /// [`Str`]:  ir/enum.Target.html#variant.Str
    /// [`Dtr`]:  ir/enum.Target.html#variant.Dtr
    /// [`Cntr`]: ir/enum.Target.html#variant.Cntr
    /// [`Otr`]:  ir/enum.Target.html#variant.Otr
    pub fn read_register(&mut self, target: ir::Target) -> Result<Vec<u8>, Error<SpiError>> {
        let ir = ir::InstructionRegister {
            target,
            action: Action::Read,
        };
        let mut tx_buffer: Vec<u8> = vec![ir.encode(), 0x00, 0x00, 0x00, 0x00];

        let result = self.interface.transfer(&mut tx_buffer)?;
        Ok(Vec::from(result))
    }
    pub fn get_status(&mut self) -> Result<Str, Error<SpiError>> {
        let raw_result = self.read_register(ir::Target::Str)?;
        let result = Str::decode(raw_result[4]);
        match result {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::EncodeError(error)),
        }
    }

    /// Reads the chip's current count, sets the sign bit appropriate to the status register
    pub fn get_count(&mut self) -> Result<i64, Error<SpiError>> {
        let raw_result = self.read_register(ir::Target::Cntr)?;
        let status = self.get_status()?;
        let count = utilities::vec_to_i64(&raw_result[1..]);
        match status.sign_bit {
            str_register::SignBit::Negative => Ok(count * -1),
            str_register::SignBit::Positive => Ok(count),
        }
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
