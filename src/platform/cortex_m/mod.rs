use cortex_m_semihosting::export::hstdout_str;

pub use self::common::main_loop;

pub use self::graphics::*;

#[path = "../common/mod.rs"]
mod common;
mod graphics;

/// Outputs a message to the console.
pub fn log(msg: &str) {
    hstdout_str(msg).unwrap();
}

