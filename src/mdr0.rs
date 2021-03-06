//! Primary configuration
//!
//! Public representation is via the [`Mdr0`] struct.
//!
//! Exposes configurations for
//!  - [`Quadrature count mode`]: Controls the interpretation of the input encoder signals
//!  - [`Index pin behavior`]: Controls the behavior of the Index pin.
//!  - [`Count cycle mode`]: Controls the behavior of the counter.
//!  - [`Filter clock division factor`]: Controls the rate of the Filter clock, for validating Index inputs.
//!
//! [`Mdr0`]: ./struct.Mdr0.html
//! [`Quadrature count mode`]: ./enum.QuadCountMode.html
//! [`Index pin behavior`]: ./enum.IndexMode.html
//! [`Count cycle mode`]: ./enum.CycleCountMode.html
//! [`Filter clock division factor`]: ./enum.FilterClockDivisionFactor.html

use bitfield::bitfield;

use crate::errors::EncoderError;
use crate::traits::{Decodable, Encodable};

#[derive(Debug)]
/// Possible quadrature count modes
pub enum QuadCountMode {
    NonQuad,
    /// (1 count per quadrature cycle)
    Quad1x,
    /// (2 count per quadrature cycle)
    Quad2x,
    /// (4 count per quadrature cycle)
    Quad4x,
}

#[derive(Debug)]
/// controls the behavior of the `Index` pin on the chip.
pub enum IndexMode {
    /// disables input on the `index` pin.
    DisableIndex,
    /// configure index as the "load CNTR" input (transfers DTR to CNTR)
    LoadCntr,
    /// configure index as the "reset CNTR" input (clears CNTR to 0).
    ClearCntr,
    /// configure index as the "load OTR" input ( transfers CNTR to OTR ).
    LoadOtr,
}

#[derive(Debug)]
/// Enum representing cycle count modes.
pub enum CycleCountMode {
    /// Free running count mode.
    FreeRunning,
    /// single-cycle count mode
    /// (counter disabled with carry or borrow, re-enabled with reset or load).
    SingleCycle,
    /// range-limit count mode
    /// (up and down count-ranges limited between DTR and zero, respectively;
    /// counting freezes at these limits but resumes when direction reverses. )
    RangeLimit,
    /// Modulo-N count mode
    /// (input clock frequency divided by factor of (n+1) where n=DTR,
    /// in both up and down directions.)
    ModuloN,
}

#[derive(Debug)]
/// Controls Filter clock frequency, used to validate Index inputs.
pub enum FilterClockDivisionFactor {
    /// Filter clock division factor = 1
    One,
    /// Filter clock division factor = 2
    Two,
}

#[derive(Debug)]
/// Representation of the Mdr0 register.
pub struct Mdr0 {
    /// Quadrature count mode
    pub quad_count_mode: QuadCountMode,
    /// Cycle behavior
    pub cycle_count_mode: CycleCountMode,
    /// Behavior of the index pin.
    pub index_mode: IndexMode,
    /// Flag for inverting the Index pin.
    pub is_index_inverted: bool,
    /// Index Filter frequency.
    pub filter_clock: FilterClockDivisionFactor,
}


bitfield! {
    struct Mdr0Payload(u8);
    impl Debug;
    pub quad_count_mode, set_quad_count_mode: 2,0;
    pub cycle_count_mode, set_cycle_count_mode: 3,2;
    pub index_mode, set_index_mode: 5,4;
    pub is_index_inverted, set_is_index_inverted: 6;
    pub filter_clock_division_factor, set_filter_clock_division_factor: 7;
}

impl Encodable for QuadCountMode {
    fn encode(&self) -> u8 {
        match self {
            QuadCountMode::NonQuad => 0b00,
            QuadCountMode::Quad1x => 0b01,
            QuadCountMode::Quad2x => 0b10,
            QuadCountMode::Quad4x => 0b11,
        }
    }
}

impl Decodable for QuadCountMode {
    fn decode(raw: u8) -> Result<Self, EncoderError> {
        match raw {
            0b00 => Ok(QuadCountMode::NonQuad),
            0b01 => Ok(QuadCountMode::Quad1x),
            0b10 => Ok(QuadCountMode::Quad2x),
            0b11 => Ok(QuadCountMode::Quad4x),
            _ => Err(EncoderError::FailedDecode),
        }
    }
}

impl Encodable for IndexMode {
    fn encode(&self) -> u8 {
        match self {
            IndexMode::DisableIndex => 0b00,
            IndexMode::LoadCntr => 0b01,
            IndexMode::ClearCntr => 0b10,
            IndexMode::LoadOtr => 0b11,
        }
    }
}

impl Decodable for IndexMode {
    fn decode(raw: u8) -> Result<IndexMode, EncoderError> {
        match raw {
            0b00 => Ok(IndexMode::DisableIndex),
            0b01 => Ok(IndexMode::LoadCntr),
            0b10 => Ok(IndexMode::ClearCntr),
            0b11 => Ok(IndexMode::LoadOtr),
            _ => Err(EncoderError::FailedDecode),
        }
    }
}

impl Encodable for CycleCountMode {
    fn encode(&self) -> u8 {
        match self {
            CycleCountMode::FreeRunning => 0b00,
            CycleCountMode::SingleCycle => 0b01,
            CycleCountMode::RangeLimit => 0b10,
            CycleCountMode::ModuloN => 0b11,
        }
    }
}

impl Decodable for CycleCountMode {
    fn decode(raw: u8) -> Result<CycleCountMode, EncoderError> {
        match raw {
            0b00 => Ok(CycleCountMode::FreeRunning),
            0b01 => Ok(CycleCountMode::SingleCycle),
            0b10 => Ok(CycleCountMode::RangeLimit),
            0b11 => Ok(CycleCountMode::ModuloN),
            _ => Err(EncoderError::FailedDecode),
        }
    }
}

impl Encodable for FilterClockDivisionFactor {
    fn encode(&self) -> u8 {
        match self {
            FilterClockDivisionFactor::One => 0b0,
            FilterClockDivisionFactor::Two => 0b1,
        }
    }
}

impl FilterClockDivisionFactor {
    pub fn decode(raw: bool) -> Result<FilterClockDivisionFactor, EncoderError> {
        match raw {
            false => Ok(FilterClockDivisionFactor::One),
            true => Ok(FilterClockDivisionFactor::Two),
        }
    }
}

impl Encodable for Mdr0 {
    fn encode(&self) -> u8 {
        let mut payload = Mdr0Payload(0x00);
        let quad_value = self.quad_count_mode.encode();
        payload.set_quad_count_mode(quad_value);
        payload.set_cycle_count_mode(self.cycle_count_mode.encode());
        payload.set_index_mode(self.index_mode.encode());
        payload.set_filter_clock_division_factor(
            match self.filter_clock {
                FilterClockDivisionFactor::One => { false }
                FilterClockDivisionFactor::Two => { true }
            }
        );

        payload.0
    }
}

impl Decodable for Mdr0 {
    fn decode(raw: u8) -> Result<Mdr0, EncoderError> {
        let payload = Mdr0Payload(raw);
        Ok(Mdr0 {
            quad_count_mode: QuadCountMode::decode(payload.quad_count_mode())?,
            cycle_count_mode: CycleCountMode::decode(payload.cycle_count_mode())?,
            index_mode: IndexMode::decode(payload.index_mode())?,
            is_index_inverted: payload.is_index_inverted(),
            filter_clock: FilterClockDivisionFactor::decode(payload.filter_clock_division_factor())?,
        })
    }
}