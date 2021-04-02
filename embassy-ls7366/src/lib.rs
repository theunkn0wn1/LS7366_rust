#![no_std]

use embassy_traits::spi::FullDuplex;

use ls7366::{ir, mdr0, mdr1};

use core::pin::Pin;

struct Ls7366<SPI> {
    iface: SPI
}

impl<SPI> Ls7366<SPI>
    where SPI: FullDuplex<u8> {
    pub fn new(iface: SPI) -> Self {
        let mut driver = Self { iface };
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
        driver
    }


    async fn write_register<'a>(self: Pin<&'a mut Self>, target: ir::Target, data: &'a [u8]) {
        self.iface.write(data).await?;
    }


}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
