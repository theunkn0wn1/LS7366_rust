use bitfield::bitfield;

pub mod mdr0;
pub mod traits;
pub mod ir;
bitfield! {
    pub struct RawMessage(u8);
    impl Debug;
    pub discarded, set_discarded: 0,2;
    pub register, set_register: 3,5;
    pub action, set_action: 6,7;
    }
