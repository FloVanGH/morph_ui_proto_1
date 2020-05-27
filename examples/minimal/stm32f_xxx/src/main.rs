#![no_std]
#![no_main]

#[path = "../../minimal.rs"]
mod minimal;

use stm32f1xx_hal as hal;
use cortex_m_rt as rt;

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};

use hal::delay::Delay;
use hal::prelude::*;
use hal::spi::{Mode, Phase, Polarity, Spi};
use hal::stm32;
use st7735_lcd;
use st7735_lcd::Orientation;

use morph::embedded_graphics;

use embedded_graphics::pixelcolor::Rgb565;

use panic_semihosting;


#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .pclk1(36.mhz())
        .freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // SPI1
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let rst = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let dc = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        16.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 160, 128);

    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
 
    minimal::start_example(disp);

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
