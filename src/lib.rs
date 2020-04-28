use bitfield::bitfield;

pub mod mdr0;
pub mod traits;

pub enum RegisterSelection {
    SelectMdr0,
    SelectMdr1,
    SelectDtr,
    SelectCenter,
    SelectOtr,
    SelectStr,
    SelectNone,
}

pub enum RegisterAction {
    ClearRegister,
    ReadRegister,
    WriteRegister,
    LoadRegister,
}

impl traits::Encodable for RegisterSelection {
    fn encode(&self) -> u8 {
        match self {
            RegisterSelection::SelectMdr0 => { 0b001 }
            RegisterSelection::SelectMdr1 => { 0b010 }
            RegisterSelection::SelectDtr => { 0b011 }
            RegisterSelection::SelectCenter => { 0b100 }
            RegisterSelection::SelectOtr => { 0b101 }
            RegisterSelection::SelectStr => { 0b110 }
            RegisterSelection::SelectNone => { 0b111 }
        }
    }
}

impl traits::Encodable for RegisterAction {
    fn encode(&self) -> u8 {
        match self {
            RegisterAction::ClearRegister => { 0b00 }
            RegisterAction::ReadRegister => { 0b01 }
            RegisterAction::WriteRegister => { 0b10 }
            RegisterAction::LoadRegister => { 0b11 }
        }
    }
}

bitfield! {
    pub struct RawMessage(u8);
    impl Debug;
    pub discarded, set_discarded: 0,2;
    pub register, set_register: 3,5;
    pub action, set_action: 6,7;
    }
