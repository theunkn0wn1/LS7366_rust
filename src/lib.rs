
pub mod ls7366 {
    use bitfield::bitfield;

    pub enum TargetRegister {
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

    bitfield! {
    pub struct RawMessage(u8);
    impl Debug;
    pub discarded, set_discarded: 0,2;
    pub selected_register, set_selected_register: 3,5;
    pub selected_action, set_selected_action: 6,7;
    }
}