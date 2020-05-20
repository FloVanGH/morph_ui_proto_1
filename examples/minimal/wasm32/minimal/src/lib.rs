use wasm_bindgen::prelude::*;

#[path = "../../../minimal.rs"]
mod minimal;

#[wasm_bindgen(start)]
pub fn start() {
    minimal::start_example();
}
