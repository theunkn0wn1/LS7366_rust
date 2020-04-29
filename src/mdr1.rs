use bitfield::bitfield;

use crate::errors::EncoderError;
use crate::traits::{Decodable, Encodable};

#[derive(Debug)]
pub enum CounterMode {
    Byte4,
    Byte3,
    Byte2,
    Byte1,
}

#[derive(Debug)]
pub struct Mdr1 {
    counter_mode: CounterMode,
    enable_counting: bool,
    flag_on_idx: bool,
    flag_on_cmp: bool,
    flag_on_bw: bool,
    flag_on_cy: bool,
}

bitfield! {
    struct Payload(u8);
    impl Debug;
    pub counter_mode, set_counter_mode: 1,0;
    pub counting_enabled, set_counting_enabled: 2;
    // bit 3 is unused
    pub flag_on_idx, set_flag_on_idx: 4;
    pub flag_on_cmp, set_flag_on_cmp: 5;
    pub flag_on_bw, set_flag_on_bw: 6;
    pub flag_on_cy, set_flag_on_cy: 7;
}
impl Encodable for CounterMode {
    fn encode(&self) -> u8 {
        match self {
            CounterMode::Byte4 => 0b00,
            CounterMode::Byte3 => 0b01,
            CounterMode::Byte2 => 0b10,
            CounterMode::Byte1 => 0b11,
        }
    }
}

impl Decodable for CounterMode {
    fn decode(raw: u8) -> Result<Self, EncoderError> {
        match raw
        {
            0b00 => Ok(CounterMode::Byte4),
            0b01 => Ok(CounterMode::Byte3),
            0b10 => Ok(CounterMode::Byte2),
            0b11 => Ok(CounterMode::Byte1),
            _ => Err(EncoderError::FailedDecode("failed to decode CounterMode".to_string()))
        }
    }
}

impl Encodable for Mdr1 {
    fn encode(&self) -> u8 {
        let mut payload = Payload(0x00);
        payload.set_counter_mode(self.counter_mode.encode());
        payload.set_counting_enabled(self.enable_counting);
        payload.set_flag_on_idx(self.flag_on_idx);
        payload.set_flag_on_cmp(self.flag_on_cmp);
        payload.set_flag_on_bw(self.flag_on_bw);
        payload.set_flag_on_cy(self.flag_on_cy);

        payload.0
    }
}

impl Decodable for Mdr1 {
    fn decode(raw: u8) -> Result<Self, EncoderError> {
        let payload = Payload(raw);

        Ok(Self {
            counter_mode: CounterMode::decode(payload.counter_mode())?,
            enable_counting: payload.counting_enabled(),
            flag_on_idx: payload.flag_on_idx(),
            flag_on_cmp: payload.flag_on_cmp(),
            flag_on_bw: payload.flag_on_bw(),
            flag_on_cy: payload.flag_on_cy(),
        })
    }
}