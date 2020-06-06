pub use morph::theme::Theme;
use morph::{
    canvas_display::CanvasDisplay,
    platform::{main_loop, CanvasBackend},
    widgets::Image,
    result::*
};

use wasm_bindgen::prelude::*;

#[path = "../../counter.rs"]
mod counter;

pub fn image() -> MorphResult<morph::widgets::Image> {
    Image::new(include_bytes!("../../../../assets/ferris.raw"), 86, 64)
}

#[wasm_bindgen(start)]
pub fn start() {
    let mut shell = counter::shell(CanvasBackend::new(CanvasDisplay::new(160, 128, "canvas").unwrap()));

    main_loop(move || {
        let result = shell.run();

        if let Ok(running) = result {
            if !running {
                return false;
            }
        }

        if let Err(err) = result {
            morph::log(format!("{:?}", err).as_str());
            return false;
        }

        true
    });
}
