use std::{cell::RefCell, rc::Rc};

mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

use crate::result::MorphResult;

pub use self::graphics::*;

mod graphics;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

/// Outputs a message to the web console.
pub fn log(msg: &str) {
    web_sys::console::log_1(&msg.into());
}

/// Platform dependent main loop.
pub fn main_loop<R: FnMut(&mut bool) -> MorphResult<()> + 'static >(mut run: R) -> MorphResult<()> {
    utils::set_panic_hook();

    let err = Rc::new(RefCell::new(Ok(())));
    let c_err = err.clone();
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut running = true;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let result = run(&mut running);

        if result.is_err() {
            *c_err.borrow_mut() = result;
            return;
        }
       
        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
       
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    let result = err.borrow();
    result.clone()
}
