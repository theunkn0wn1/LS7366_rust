#![no_std]

use embassy_traits::spi::FullDuplex;

use ls7366::traits::Decodable;
use ls7366::Error;
use ls7366::{ir, mdr0, mdr1, Encodable};

use core::pin::Pin;

pub struct Ls7366<SPI> {
    iface: SPI,
}

impl<SPI, SpiError> Ls7366<SPI>
where
    SPI: FullDuplex<u8, Error = SpiError>,
{
    pub fn new_uninit(iface: SPI) -> Self {
        Self { iface }
    }

    pub async fn init<'a>(mut self: Pin<&'a mut Self>) -> Result<(), Error<SpiError>> {
        // Creating configurations for the two MDR configuration registers
        let mdr0_payload = mdr0::Mdr0 {
            quad_count_mode: mdr0::QuadCountMode::Quad4x,
            cycle_count_mode: mdr0::CycleCountMode::FreeRunning,
            index_mode: mdr0::IndexMode::DisableIndex,
            is_index_inverted: false,
            filter_clock: mdr0::FilterClockDivisionFactor::One,
        };
        let mdr1_payload = mdr1::Mdr1 {
            counter_mode: mdr1::CounterMode::Byte4,
            disable_counting: false,
            flag_on_idx: false,
            flag_on_cmp: false,
            flag_on_bw: false,
            flag_on_cy: false,
        };

        self.as_mut()
            .write_register(ir::Target::Mdr0, &[mdr0_payload.encode()])
            .await?;
        self.as_mut()
            .write_register(ir::Target::Mdr1, &[mdr1_payload.encode()])
            .await?;
        Ok(())
    }

    /// Writes data to specified register
    ///
    /// NOTE(safety) safe as Self is pinned.
    async fn write_register<'a>(
        self: Pin<&'a mut Self>,
        target: ir::Target,
        data: &'a [u8],
    ) -> Result<(), Error<SpiError>> {
        let ir_cmd = ir::InstructionRegister {
            target,
            action: ir::Action::Write,
        };
        if data.len() > 4 {
            return Err(Error::PayloadTooBig);
        }

        let encoded = ir_cmd.encode();
        // create payload, initializing it with the IR command
        let payload: &mut [u8] = &mut [encoded, encoded, encoded, encoded, encoded];

        let mut i = 1;
        // copy data bytes into the payload
        for datum in data {
            payload[i] = *datum;
            i += 1;
        }

        let result: Result<(), SpiError> = unsafe {
            Pin::new_unchecked(&mut self.get_unchecked_mut().iface)
                .write(data)
                .await
        };
        // any error we get us an underlying error, therefore we map it to that type.
        result.map_err(|e| Error::SpiError(e))
    }

    /// Executes a read operation against specified register, returning up to 4 bytes from the chip.
    ///
    /// ## Note:
    ///
    /// Reading from [`Str`] clears the register to zero.
    ///
    /// Reading from [`Dtr`] is a Noop.
    ///
    /// Reading from [`Cntr`] overwrites [`Otr`].
    ///
    /// [`Str`]:  ir/enum.Target.html#variant.Str
    /// [`Dtr`]:  ir/enum.Target.html#variant.Dtr
    /// [`Cntr`]: ir/enum.Target.html#variant.Cntr
    /// [`Otr`]:  ir/enum.Target.html#variant.Otr
    pub async fn read_register<'a>(
        self: Pin<&'a mut Self>,
        rx_buffer: &'a mut [u8],
        target: ir::Target,
    ) -> Result<&'a [u8], Error<SpiError>> {
        let ir = ir::InstructionRegister {
            target,
            action: ls7366::Action::Read,
        };
        let tx_buffer = &mut [ir.encode(), 0x00, 0x00, 0x00, 0x00];

        // do transfer operaton.
        // this will modify the RX buffer and return nothing but a possible error.
        unsafe {
            Pin::new_unchecked(&mut self.get_unchecked_mut().iface)
                .read_write(tx_buffer, rx_buffer)
                .await?
        };
        // If we got this far, then transfer succeeded.
        Ok(rx_buffer)
    }

    pub async fn get_status(
        self: Pin<&mut Self>,
    ) -> Result<ls7366::str_register::Str, Error<SpiError>> {
        let result: &mut [u8] = &mut [0x00, 0x00, 0x00, 0x00];
        let raw_result = self.read_register(result, ir::Target::Str).await?;
        ls7366::str_register::Str::decode(raw_result[3]).map_err(|e| Error::EncodeError(e))
    }
    // pub async fn get_count(
    //     mut self: Pin<&mut Self>
    // ) -> Result<i64, Error<SpiError>>{
    //     let buffer: &mut [u8] = &mut [0x00, 0x00, 0x00, 0x00];
    //     let raw_result = self.as_mut().read_register(buffer, ir::Target::Cntr).await?;
    //     let status = self.as_mut().get_status().await?;
    //     let count = ls7366::utilities::vec_to_i64(&raw_result);
    //     match status.sign_bit {
    //         ls7366::str_register::SignBit::Negative => Ok(count * -1),
    //         ls7366::str_register::SignBit::Positive => Ok(count),
    //     }
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
