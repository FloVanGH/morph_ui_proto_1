#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use morph::prelude::*;

#[entry]
fn main() -> ! {
 
    hprintln!("Hello, world!").unwrap();

    let mut shell = Shell {};
    shell.start();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}