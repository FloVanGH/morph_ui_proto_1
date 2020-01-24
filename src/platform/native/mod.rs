/// Platform dependent main loop.
pub fn main_loop<R: FnMut()>(mut run: R) {
    loop {
        run();
    }
}