#![no_std]
#![no_main]
#![feature(trait_alias)]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]

use embassy::executor::Spawner;
use embassy_stm32::{
    hal::{
        dma::{Channel3, Stream3, Stream5, StreamsTuple},
    },
    pac::{DMA2, SPI1},

    interrupt, spi,
};
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::prelude::*;

type _SPI1 = spi::Spi<SPI1, Stream5<DMA2>, Stream3<DMA2>, Channel3>;

#[embassy::main]
async fn main(spawner: Spawner) {
    rtt_init_print!();
    rprintln!("hello, world!");
    let (dp, clocks) = embassy_stm32::Peripherals::take().expect("failed to take peripherals");
    // workaround for WFI errata.
    dp.DBGMCU.cr.modify(|_, w| {
        w.dbg_sleep().set_bit();
        w.dbg_standby().set_bit();
        w.dbg_stop().set_bit()
    });

    let dma2_streams = StreamsTuple::new(dp.DMA2);

    // grab PA bus.
    let pa = dp.GPIOA.split();

    // configure pins
    let spi1_mosi = pa.pa7.into_alternate_af5();
    let spi1_miso = pa.pa6.into_alternate_af5();
    let spi1_sck = pa.pa5.into_alternate_af5();
    let spi1_nss = pa.pa4.into_alternate_af5();

    let spi = unsafe {
        embassy_stm32::spi::Spi::new(
            dp.SPI1,
            (dma2_streams.5, dma2_streams.2),
            (spi1_sck,spi1_miso, spi1_mosi),
            interrupt::take!(DMA2_STREAM5),
            interrupt::take!(DMA2_STREAM2),
            interrupt::take!(SPI1),
            embassy_stm32::hal::spi::Mode {
                polarity: embassy_stm32::hal::spi::Polarity::IdleLow,
                phase: embassy_stm32::hal::spi::Phase::CaptureOnFirstTransition,
            },
            100.hz(),
            (clocks),
        )
    };
    let driver = embassy_ls7366::Ls7366::new_uninit(spi);


}
