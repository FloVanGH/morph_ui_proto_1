use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::{geometry::Size, graphics, result::*};

/// The `RenderContext` provides different draw methods.
pub struct RenderContext {
    size: Size,
}

impl RenderContext {
    /// Creates a new render context from the given size.
    pub fn new(size: Size) -> MorphResult<Self> {
        Ok(RenderContext { size })
    }
}

impl graphics::RenderContext for RenderContext {

}
