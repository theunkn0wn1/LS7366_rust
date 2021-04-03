#![no_std]
#![no_main]
#![feature(trait_alias)]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]

use embassy::executor::Spawner;
use embassy_stm32::{
    hal::prelude::*,
    hal::dma::{Channel3, Stream3, Stream5, StreamsTuple},
    interrupt,
    pac::{DMA2, SPI1},
    spi,
};
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

use futures::pin_mut;


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
    let pb = dp.GPIOB.split();

    // configure pins
    let spi1_mosi = pa.pa7.into_alternate_af5();
    let spi1_miso = pa.pa6.into_alternate_af5();
    let spi1_sck = pa.pa5.into_alternate_af5();
    let mut spi1_ss1 = pb.pb8.into_push_pull_output();
    // let spi1_nss = pa.pa4.into_alternate_af5();

    let spi = unsafe {
        embassy_stm32::spi::Spi::new(
            dp.SPI1,
            (dma2_streams.5, dma2_streams.2),
            (spi1_sck, spi1_miso, spi1_mosi),
            interrupt::take!(DMA2_STREAM5),
            interrupt::take!(DMA2_STREAM2),
            interrupt::take!(SPI1),
            embassy_stm32::hal::spi::Mode {
                polarity: embassy_stm32::hal::spi::Polarity::IdleLow,
                phase: embassy_stm32::hal::spi::Phase::CaptureOnFirstTransition,
            },
            100.hz(),
            clocks,
        )
    };
    rprintln!("insantiating driver...");
    let driver = embassy_ls7366::Ls7366::new_uninit(spi);
    // let spi = stm32f4xx_hal::spi::Spi::spi1(
    //     dp.SPI1,
    //     (spi1_sck, spi1_miso, spi1_mosi),
    //     stm32f4xx_hal::spi::Mode {
    //         polarity: stm32f4xx_hal::spi::Polarity::IdleLow,
    //         phase: stm32f4xx_hal::spi::Phase::CaptureOnFirstTransition
    //     },
    //     14_000.hz(),
    //     clocks
    //
    // );
    // let mut driver = ls7366::Ls7366::new(spi).expect("failed to spawn driver");
    pin_mut!(driver);

    // spi1_ss1.set_high().expect("failed to set SS1");
    spi1_ss1.set_low().expect("failed to set SS1");


    // // this has to be done as a seperate operation thanks to the pin.
    rprintln!("initializing driver...");
    spi1_ss1.set_high().expect("failed to set SS1");
    driver.as_mut().init().await.expect("failed to init chip.");
    spi1_ss1.set_low().expect("failed to set SS1");

    rprintln!("fetching status...");
    spi1_ss1.set_high().expect("failed to set SS1");
    let state = driver.as_mut().get_status().await.expect("failed to get status");
    spi1_ss1.set_low().expect("failed to set SS1");

    // let spi1 = driver.reclaim();
    // spi1.free();



    rprintln!("status := {:?}", state);
}
