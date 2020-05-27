#![no_std]
#![no_main]

#[path = "../../minimal.rs"]
mod minimal;

use panic_halt as _;

use stm32f1xx_hal::{
    pac,
    prelude::*,
    spi::{Mode, Phase, Polarity, Spi},
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // // Get access to the device specific peripherals from the peripheral access crate
    // let dp = pac::Peripherals::take().unwrap();

    // // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // // HAL structs
    // let mut flash = dp.FLASH.constrain();
    // let mut rcc = dp.RCC.constrain();

    // // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // // `clocks`
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // // Acquire the GPIOA peripheral
    // let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // let pins = (
    //     gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh),
    //     gpiob.pb14.into_floating_input(&mut gpiob.crh),
    //     gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh),
    // );

    // let spi_mode = Mode {
    //     polarity: Polarity::IdleLow,
    //     phase: Phase::CaptureOnFirstTransition,
    // };
    // let mut spi = Spi::spi2(dp.SPI2, pins, spi_mode, 100.khz(), clocks, &mut rcc.apb1);

    // let mut disp = st7735_lcd::ST7735::new(
    //     spi,
    //     pins.0,
    //     gpiob.nrst.into_alternate_push_pull(&mut gpiob.crh),
    //     false,
    //     true,
    // );
    minimal::start_example();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
