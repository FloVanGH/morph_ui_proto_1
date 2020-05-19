use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::geometry::Size;

/// The `RenderContext` provides different draw methods.
pub struct RenderContext {
    size: Size,
}

impl RenderContext {
    /// Creates a new render context from the given size.
    pub fn new(size: Size) -> Self {
        RenderContext { size }
    }
}
