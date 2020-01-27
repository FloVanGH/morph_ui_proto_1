use cortex_m_semihosting::hprintln;

/// Outputs a message to the console.
pub fn log(msg: &str) {
    // hprintln!(msg.as_bytes()).unwrap();
}

/// Platform dependent main loop.
pub fn main_loop<R: FnMut()>(mut run: R) {
    loop {
        run();
    }
}