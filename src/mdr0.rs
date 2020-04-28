use crate::traits::Encodable;
use bitfield::bitfield;
/// Possible quadrature count modes
pub enum NonQuadCountMode {
    NonQuad,
    /// (1 count per quadrature cycle)
    Quad1x,
    /// (2 count per quadrature cycle)
    Quad2x,
    /// (4 count per quadrature cycle)
    Quad4x,
}

pub enum IndexMode {
    DisableIndex,
    /// configure index as the "load CNTR" input (transfers DTR to CNTR)
    LoadCntr,
    /// configure index as the "reset CNTR" input (clears CNTR to 0).
    ClearCntr,
    /// configure index as the "load OTR" input ( transfers CNTR to OTR ).
    LoadOtr,
}

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


bitfield! {
    struct Mdr0Payload(u8);
    impl Debug;
    pub quad_count_mode, set_quad_count_mode: 0,2;
    pub free_running_count_mode, set_free_running_count_mode: 2,3;
    pub index_mode, set_index_mode: 4,5;
    pub is_index_inverted, set_is_index_inverted: 6;
    pub filter_clock_division_factor, set_filter_clock_division_factor: 7;
}

impl Encodable for CountMode {
    fn encode(&self) -> u8 {
        match self {
            CountMode::NonQuad => { 0b00 }
            CountMode::Quad1x => { 0b01 }
            CountMode::Quad2x => { 0b10 }
            CountMode::Quad4x => { 0b11 }
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