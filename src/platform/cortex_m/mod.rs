use cortex_m_semihosting::export::hstdout_str;

use crate::result::MorphResult;

/// Outputs a message to the console.
pub fn log(msg: &str) {
    hstdout_str(msg).unwrap();
}

/// Platform dependent main loop.
pub fn main_loop<R: FnMut() -> MorphResult<()>>(mut run: R) -> MorphResult<()> {
    loop {
        let result = run();
        if result.is_err() {
            return result;
        }
    }
}
