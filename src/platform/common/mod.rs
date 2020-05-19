use crate::result::MorphResult;

/// Platform dependent main loop.
pub fn main_loop<R: FnMut(&mut bool) -> MorphResult<()>>(mut run: R) -> MorphResult<()> {
    let mut running = true;
    loop {
        let result = run(&mut running);
        if result.is_err() || !running {
            return result;
        }
    }
}
