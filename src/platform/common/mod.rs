use crate::result::MorphResult;

/// Platform dependent main loop.
pub fn main_loop<R: FnMut() -> MorphResult<()>>(mut run: R) -> MorphResult<()> {
    loop {
        let result = run();
        if result.is_err() {
            return result;
        }
    }
}
