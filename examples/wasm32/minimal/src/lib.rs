use wasm_bindgen::prelude::*;

#[path = "../../../minimal.rs"]
mod minimal;

#[wasm_bindgen]
pub fn start() {
    minimal::start_example();
}