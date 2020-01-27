#![no_std]
#![no_main]

#[path = "minimal.rs"]
mod minimal;

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
 
    hprintln!("Hello, world!").unwrap();

    minimal::start_example();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}