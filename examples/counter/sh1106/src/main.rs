#![no_std]
#![no_main]

#[path = "../../counter.rs"]
mod counter;

use cortex_m_rt::{entry, exception, ExceptionFrame};
pub use morph::theme::binary::BinaryTheme as Theme;
use morph::{
    embedded_graphics::{
        fonts::{Font6x8, Text},
        pixelcolor::BinaryColor,
        prelude::*,
        style::TextStyle,
    },
    platform::Sh1106Backend,
    prelude::*,
};
use panic_semihosting as _;
use sh1106::{prelude::*, Builder};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    let mut shell = counter::shell(Sh1106Backend::new(disp));

    // loop {
    let result = shell.run();

    if let Err(err) = result {
        panic!("{:?}", err);
    }
    // }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
