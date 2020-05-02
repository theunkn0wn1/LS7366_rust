#[cfg(test)]
mod tests {
    use std::error::Error;

    use embedded_hal_mock::spi::{Mock, Transaction as SpiTransaction};

    use ls7366::{Action, Encodable, Target};
    use ls7366::ir::InstructionRegister;
    use ls7366::Ls7366;
    use ls7366::str_register;

    #[test]
    fn test_get_count() {
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
            ),
            SpiTransaction::transfer(vec![InstructionRegister {
                target: Target::Cntr,
                action: Action::Read,
            }.encode(), 0x00, 0x00, 0x00, 0x00], vec![0x00, 0xDE, 0xAD, 0xBE, 0xEF]),
            // STR read, will return negative sign
            SpiTransaction::transfer(vec![InstructionRegister {
                target: Target::Str,
                action: Action::Read,
            }.encode(), 0x00, 0x00, 0x00, 0x00], vec![0x00, 0x00, 0x00, 0x00, 0b00001011],
            )
        ];

        let spi = Mock::new(&expectations);
        let mut driver = Ls7366::new_uninit(spi);

        let result = driver.get_count().unwrap();

        assert_eq!(result, 0xDEADBEEF);
        assert_eq!(driver.get_count().unwrap(), -0xDEADBEEF);
    }

    #[test]
    fn test_status() {
        let expectations = [
            // STR read, will return positive sign
            SpiTransaction::transfer(vec![InstructionRegister {
                target: Target::Str,
                action: Action::Read,
            }.encode(), 0x00, 0x00, 0x00, 0x00], vec![0x00, 0x00, 0x00, 0x00, 0b00001010],
            ),
            // STR read, will return negative sign
            SpiTransaction::transfer(vec![InstructionRegister {
                target: Target::Str,
                action: Action::Read,
            }.encode(), 0x00, 0x00, 0x00, 0x00], vec![0x00, 0x00, 0x00, 0x00, 0b11110101],
            )
        ];
        let expected_results = [
            str_register::Str {
                cary: false,
                borrow: false,
                compare: false,
                index: false,
                count_enabled: true,
                power_loss: false,
                count_direction: str_register::Direction::Up,
                sign_bit: str_register::SignBit::Positive,
            },
            str_register::Str {
                cary: true,
                borrow: true,
                compare: true,
                index: true,
                count_enabled: false,
                power_loss: true,
                count_direction: str_register::Direction::Down,
                sign_bit: str_register::SignBit::Negative,
            }
        ];
        let spi = Mock::new(&expectations);
        let mut driver = Ls7366::new_uninit(spi);

        for payload in expected_results.iter() {
            let result = driver.get_status().unwrap();
            assert_eq!(&result, payload);
        }
    }

    #[test]
    fn test_write_register() {
        let expectations = [
            // Dtr write
            SpiTransaction::write(vec![InstructionRegister {
                target: Target::Dtr,
                action: Action::Write,
            }.encode(), 0xBA, 0xAD, 0xBE, 0xEF],
            ),

            // mdr0 write
            SpiTransaction::write(vec![InstructionRegister {
                target: Target::Mdr0,
                action: Action::Write,
            }.encode(), 0xFD, 0xFD, 0xFD, 0xFD],
            ),
        ];

        let spi = Mock::new(&expectations);
        let mut driver = Ls7366::new_uninit(spi);

        driver.write_register(Target::Dtr, &vec![0xBA, 0xAD, 0xBE, 0xEF]).unwrap();
        driver.write_register(Target::Mdr0, &vec![0xFD, 0xFD, 0xFD, 0xFD]).unwrap();
    }
}
