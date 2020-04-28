use crate::traits::Encodable;

/// Possible quadrature count modes
pub enum CountMode {
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
    loadCNTR,
    /// configure index as the "reset CNTR" input (clears CNTR to 0).
    clearCNTR,
    /// configure index as the "load OTR" input ( transfers CNTR to OTR ).
    loadOTR,
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
            IndexMode::loadCNTR => { 0b01 }
            IndexMode::clearCNTR => { 0b10 }
            IndexMode::loadOTR => { 0b11 }
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