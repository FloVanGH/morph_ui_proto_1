use cortex_m_semihosting::export::hstdout_str;

pub use self::common::main_loop;

pub use self::sh1106::*;

mod sh1106;

#[path = "../common/mod.rs"]
mod common;

/// Outputs a message to the console.
pub fn log(msg: &str) {
    hstdout_str(msg).unwrap();
    hstdout_str("\n").unwrap();
}

