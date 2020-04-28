use bitfield::bitfield;

use crate::traits::Encodable;

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
pub enum IndexMode {
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
pub enum FilterClockDivisionFactor {
    /// Filter clock division factor = 1
    One,
    /// Filter clock division factor = 2
    Two,
}
#[derive(Debug)]
pub struct Mdr0 {
    pub quad_count_mode: QuadCountMode,
    pub cycle_count_mode: CycleCountMode,
    pub index_mode: IndexMode,
    pub is_index_inverted: bool,
    pub filter_clock: FilterClockDivisionFactor,
}


bitfield! {
    pub struct Mdr0Payload(u64);
    impl Debug;
    u8;
    pub quad_count_mode, set_quad_count_mode: 0,1;
    pub cycle_count_mode, set_cycle_count_mode: 2,3;
    pub index_mode, set_index_mode: 4,5;
    pub is_index_inverted, set_is_index_inverted: 6;
    pub filter_clock_division_factor, set_filter_clock_division_factor: 7;
}

impl Encodable for QuadCountMode {
    fn encode(&self) -> u8 {
        match self {
            QuadCountMode::NonQuad => { 0b00 }
            QuadCountMode::Quad1x => { 0b01 }
            QuadCountMode::Quad2x => { 0b10 }
            QuadCountMode::Quad4x => { 0b11 }
        }
    }
}

impl Encodable for IndexMode {
    fn encode(&self) -> u8 {
        match self {
            IndexMode::DisableIndex => { 0b00 }
            IndexMode::LoadCntr => { 0b01 }
            IndexMode::ClearCntr => { 0b10 }
            IndexMode::LoadOtr => { 0b11 }
        }
    }
}

impl Encodable for CycleCountMode {
    fn encode(&self) -> u8 {
        match self {
            CycleCountMode::FreeRunning => { 0b00 }
            CycleCountMode::SingleCycle => { 0b01 }
            CycleCountMode::RangeLimit => { 0b10 }
            CycleCountMode::ModuloN => { 0b11 }
        }
    }
}

impl Encodable for FilterClockDivisionFactor {
    fn encode(&self) -> u8 {
        match self {
            FilterClockDivisionFactor::One => { 0b0 }
            FilterClockDivisionFactor::Two => { 0b1 }
        }
    }
}

impl Mdr0 {
    pub fn encode(&self) -> u64 {
        let mut payload = Mdr0Payload(0x00);
        let quad_value = self.quad_count_mode.encode();
        payload.set_quad_count_mode(quad_value);
        payload.set_cycle_count_mode(self.cycle_count_mode.encode());
        payload.set_index_mode(self.index_mode.encode());
        payload.set_filter_clock_division_factor(
            match self.filter_clock{
                FilterClockDivisionFactor::One => {false},
                FilterClockDivisionFactor::Two => {true},
            }
        );

        payload.0
    }
}