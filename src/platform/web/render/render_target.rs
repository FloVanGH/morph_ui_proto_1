use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::{geometry::Size, result::*};
use super::{RenderContext, super::utils};

/// The `RenderTarget` is used to draw the content of a `RenderContext` on the screen.
pub struct RenderTarget {
    size: Size,
    // canvas: HtmlCanvasElement
}

impl RenderTarget {
    /// Creates a new render target from the given size.
    pub fn new(size: Size) -> MorphResult<Self> {

        let canvas = {
            if let Some(canvas) = utils::document()?.get_element_by_id("morph_canvas") {
                Some(canvas)
            } else {
                if let Ok(canvas) = utils::document()?.create_element("canvas") {
                    utils::body()?.append_child(&canvas);
                    Some(canvas)
                }  else {
                    None
                }
            }
        };

        



        Ok(RenderTarget {
            size
        })
    }

    /// Draw the given `RenderContext` on the screen.
    pub fn draw_to_screen(&mut self, render_context: RenderContext) {

    }
}