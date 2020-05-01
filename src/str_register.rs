use bitfield::bitfield;

use crate::errors::EncoderError;
use crate::traits::Decodable;

#[derive(Debug)]
pub enum SignBit {
    Negative,
    Positive,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}


#[derive(Debug)]
pub struct Str {
    /// Carry (CNTR overflow) latch.
    pub cary: bool,
    /// Borrow (CNTR underflow) latch.
    pub borrow: bool,
    /// Compare (CNTR = DTR) latch.
    pub compare: bool,
    /// Index latch.
    pub index: bool,
    /// Counter enabled status.
    pub count_enabled: bool,
    /// Power Loss latch. Set to true with power on.
    pub power_loss: bool,
    /// Direction of count.
    pub count_direction: Direction,
    /// Sign.
    pub sign_bit: SignBit,
}
bitfield! {
    struct Payload(u8);
    impl Debug;
    pub sign, _: 0;
    pub count_direction,_: 1;
    pub power_loss, _ : 2;
    pub cary, _: 3;
    pub borrow_, _: 4;
    pub compare, _: 5;
    pub index, _: 6;
    pub count_enabled, _: 7;
}

impl SignBit {
    fn decode(raw: bool) -> Result<Self, EncoderError> {
        match raw {
            false => Ok(SignBit::Negative),
            true => Ok(SignBit::Positive),
        }
    }
}


impl Direction {
    fn decode(raw: bool) -> Result<Self, EncoderError> {
        match raw {
            false => Ok(Direction::Down),
            true => Ok(Direction::Up),
        }
    }
}


impl Decodable for Str {
    fn decode(raw: u8) -> Result<Self, EncoderError> {
        let payload = Payload(raw);
        Ok(Self {
            sign_bit: SignBit::decode(payload.sign())?,
            count_direction: Direction::decode(payload.count_direction())?,
            compare: payload.power_loss(),
            cary: payload.cary(),
            borrow: payload.borrow_(),
            power_loss: payload.compare(),
            index: payload.index(),
            count_enabled: payload.count_enabled(),
        })
    }
}
