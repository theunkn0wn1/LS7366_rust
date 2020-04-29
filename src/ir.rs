use bitfield::bitfield;

use crate::errors::EncoderError;
use crate::traits::{Decodable, Encodable};

#[derive(Debug)]
pub enum Target {
    Mdr0,
    Mdr1,
    Dtr,
    Cntr,
    Otr,
    Str,
    None,
}

#[derive(Debug)]
pub enum Action {
    Clear,
    Read,
    Write,
    Load,
}

#[derive(Debug)]
pub struct InstructionRegister {
    pub target: Target,
    pub action: Action,
}
bitfield! {
    struct Payload(u8);
    impl Debug;
    pub target, set_target: 5,3;
    pub action,set_action: 7,6;

}

impl Encodable for Target {
    fn encode(&self) -> u8 {
        match self {
            Target::Mdr0 => 0b001,
            Target::Mdr1 => 0b010,
            Target::Dtr => 0b011,
            Target::Cntr => 0b100,
            Target::Otr => 0b101,
            Target::Str => 0b110,
            Target::None => 0b111,
        }
    }
}

impl Decodable for Target {
    fn decode(raw: u8) -> Result<Self, EncoderError> {
        match raw {
            0b000 => Ok(Target::None),
            0b001 => Ok(Target::Mdr0),
            0b010 => Ok(Target::Mdr1),
            0b011 => Ok(Target::Dtr),
            0b100 => Ok(Target::Cntr),
            0b101 => Ok(Target::Otr),
            0b110 => Ok(Target::Str),
            0b111 => Ok(Target::None),
            _ => { Err(EncoderError::FailedDecode("failed to decode Target".to_string())) }
        }
    }
}

impl Encodable for Action {
    fn encode(&self) -> u8 {
        match self {
            Action::Clear => 0b00,
            Action::Read => 0b01,
            Action::Write => 0b10,
            Action::Load => 0b11,
        }
    }
}

impl Decodable for Action {
    fn decode(raw: u8) -> Result<Self, EncoderError> {
        match raw {
            0b00 => Ok(Action::Clear),
            0b01 => Ok(Action::Read),
            0b10 => Ok(Action::Write),
            0b11 => Ok(Action::Load),
            _ => Err(EncoderError::FailedDecode("failed to decode Action".to_string()))
        }
    }
}

impl Encodable for InstructionRegister {
    fn encode(&self) -> u8 {
        let mut payload = Payload(0x00);
        payload.set_target(self.target.encode());
        payload.set_action(self.action.encode());

        payload.0
    }
}

impl Decodable for InstructionRegister {
    fn decode(raw: u8) -> Result<Self, EncoderError> {
        let payload = Payload(raw);
        Ok(Self {
            target: Target::decode(payload.target())?,
            action: Action::decode(payload.action())?,
        })
    }
}