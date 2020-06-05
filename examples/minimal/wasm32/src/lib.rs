use embedded_graphics::pixelcolor::Rgb565;

use wasm_bindgen::prelude::*;

use morph::canvas_display::CanvasDisplay;

#[path = "../../minimal.rs"]
mod minimal;

#[wasm_bindgen(start)]
pub fn start() {
    morph::log(format!("{:?}", minimal::start_example::<CanvasDisplay, Rgb565>(CanvasDisplay::new(128,160, "morph_canvas").unwrap())).as_str());
}
