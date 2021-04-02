#[cfg(test)]
mod tests {
    use crate::ir::{Action, InstructionRegister, Target};
    use crate::traits::{Decodable, Encodable};

    const CLEAR_CNTR: u8 = 0b00100000;
    const LOAD_DTR: u8 = 0b11011000;
    const CLEAR_STR: u8 = 0b00110000;

    #[test]
    fn test_ir_decode() -> Result<(), String> {
        let result = InstructionRegister::decode(CLEAR_CNTR).expect("failed decode");
        match result.target {
            Target::Cntr => Ok(()),
            _ => Err("incorrect target".to_string()),
        }?;
        match result.action {
            Action::Clear => Ok(()),
            _ => Err("incorrect action".to_string()),
        }?;
        Ok(())
    }

    #[test]
    fn test_load_dtr_decode() -> Result<(), String> {
        let result = InstructionRegister::decode(LOAD_DTR).expect("failed decode");
        match result.target {
            Target::Dtr => Ok(()),
            _ => Err("incorrect target".to_string()),
        }?;
        match result.action {
            Action::Load => Ok(()),
            _ => Err("incorrect action".to_string()),
        }?;
        Ok(())
    }

    #[test]
    fn test_load_dtr_encode() {
        let ir = InstructionRegister {
            target: Target::Dtr,
            action: Action::Load,
        };
        assert_eq!(ir.encode(), LOAD_DTR);
    }

    #[test]
    fn test_ir_encode() {
        let ir = InstructionRegister {
            target: Target::Cntr,
            action: Action::Clear,
        };
        assert_eq!(ir.encode(), CLEAR_CNTR);
    }
    #[test]
    fn test_clear_str_encode() {
        let ir = InstructionRegister {
            target: Target::Str,
            action: Action::Clear,
        };
        assert_eq!(ir.encode(), CLEAR_STR)
    }
    #[test]
    fn test_clear_str_decode() -> Result<(), String> {
        let result = InstructionRegister::decode(CLEAR_STR).expect("failed decode");
        match result.target {
            Target::Str => Ok(()),
            _ => Err("incorrect target".to_string()),
        }?;
        match result.action {
            Action::Clear => Ok(()),
            _ => Err("incorrect action".to_string()),
        }?;
        Ok(())
    }
}
