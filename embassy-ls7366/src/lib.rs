#![no_std]

use embassy_traits::spi::FullDuplex;

use ls7366::{ir, mdr0, mdr1, Encodable};
use ls7366::Error;

use core::pin::Pin;
use core::future::Future;


pub mod error;

struct Ls7366<SPI> {
    iface: SPI
}

impl<SPI, SpiError> Ls7366<SPI>
    where SPI: FullDuplex<u8, Error=SpiError> {

    pub fn new_uninit(iface: SPI) -> Self {
        Self { iface }
    }

    async fn init<'a>(self: Pin<&'a mut Self>) -> Result<(), Error<SpiError>>{
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

        self.write_register(ir::Target::Mdr0, &[mdr0_payload.encode()]).await?;
        Ok(())
    }


    /// Writes data to specified register
    ///
    /// NOTE(safety) safe as Self is pinned.
    async fn write_register<'a>(mut self: Pin<&'a mut Self>, target: ir::Target, data: &'a [u8]) -> Result<(), Error<SpiError>>{

        let ir_cmd = ir::InstructionRegister {
            target,
            action: ir::Action::Write,
        };
        if data.len() > 4 {
            return Err(Error::PayloadTooBig);
        }

        let encoded = ir_cmd.encode();
        let payload: &mut [u8] = &mut [encoded, encoded, encoded, encoded, encoded];

        let mut i = 1;
        for datum in data {
            payload[i] = *datum;
            i+=1;
        }

        let result = unsafe{
             Pin::new_unchecked(&mut self.get_unchecked_mut().iface).write(data).await
        };
        match result {
            Ok(()) =>{
                Ok(())
            }
            Err(e) => {
                Err(Error::SpiError(e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
