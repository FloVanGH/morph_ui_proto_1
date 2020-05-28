use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb565};

use wasm_bindgen::prelude::*;

use morph::graphics::WasmDisplay;

#[path = "../../minimal.rs"]
mod minimal;

#[wasm_bindgen(start)]
pub fn start() {
    minimal::start_example::<WasmDisplay, Rgb565>(WasmDisplay::new(128,160).unwrap());
}
