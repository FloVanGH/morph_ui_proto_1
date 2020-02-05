#![feature(alloc)]
#![feature(alloc_error_handler)]

#![no_std]
#![no_main]

#[path = "minimal.rs"]
mod minimal;

use panic_halt as _;

use core::alloc::Layout;

use alloc_cortex_m::CortexMHeap;

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

use cortex_m::asm;
use cortex_m_semihosting::debug;
use cortex_m_rt::entry;

use st7735::color::{Color, DefaultColor};
use st7735::ST7734;

use stm32f1xx_hal::{
    prelude::*,
    pac,
    delay::Delay,
}; 

use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let cp = cortex_m::Peripherals::take().unwrap(); 
    let mut delay = Delay::new(cp.SYST, clocks);


    // Acquire the GPIOA peripheral
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // let clk = gpiob.adc5;
        
    minimal::start_example();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}