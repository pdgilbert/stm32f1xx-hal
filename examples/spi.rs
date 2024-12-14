#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh,
};

use stm32f1xx_hal::{
    gpio::{Output, PA4},
    pac::{Peripherals, SPI1},
    prelude::*,
    spi::{Mode, Phase, Polarity, Spi},
};

fn setup() -> (Spi<SPI1, u8>, PA4<Output>) {
    let dp = Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    //let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();

    // SPI1
    let sck = gpioa.pa5;
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7;
    let cs = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);

    let spi = dp
        .SPI1
        //.remap(&mut afio.mapr) // if you want to use PB3, PB4, PB5
        .spi((Some(sck), Some(miso), Some(mosi)), MODE, 1.MHz(), &clocks);

    (spi, cs)
}

#[entry]
fn main() -> ! {
    let (_spi, _cs) = setup();

    loop {}
}
