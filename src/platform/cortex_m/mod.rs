use cortex_m_semihosting::export::hstdout_str;

/// Outputs a message to the console.
pub fn log(msg: &str) {
    hstdout_str(msg).unwrap();
}

/// Platform dependent main loop.
pub fn main_loop<R: FnMut()>(mut run: R) {
    loop {
        run();
    }
}