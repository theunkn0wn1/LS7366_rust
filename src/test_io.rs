#[cfg(test)]
mod tests {
    use embedded_hal_mock::spi::{Mock, Transaction as SpiTransaction};

    use crate::{Action, Encodable, Target};
    use crate::ir::InstructionRegister;
    use crate::Ls7366;
    use std::error::Error;

    #[test]
    fn test_read() -> Result<(), Box<dyn Error>> {
        let expectations = [
            SpiTransaction::transfer(vec![InstructionRegister {
                target: Target::Cntr,
                action: Action::Read,
            }.encode(), 0x00, 0x00, 0x00, 0x00], vec![0x00, 0xDE, 0xAD, 0xBE, 0xEF]),
            // STR read, will return positive sign
            SpiTransaction::transfer(vec![InstructionRegister {
                target: Target::Str,
                action: Action::Read,
            }.encode(), 0x00, 0x00, 0x00, 0x00], vec![0x00, 0x00, 0x00, 0x00, 0b00001010],
            )
        ];

        let mut spi = Mock::new(&expectations);
        let mut driver = Ls7366::new_uninit(spi);

        let result = driver.get_count()?;

        assert_eq!(result, 0xDEADBEEF);
        Ok(())
    }
}
