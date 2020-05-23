use std::any::Any;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::{super::utils, RenderContext};
use crate::{geometry::*, graphics, result::*};

/// The `RenderTarget` is used to draw the content of a `RenderContext` on the screen.
#[derive(Clone, Debug, PartialEq)]
pub struct RenderTarget {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

impl RenderTarget {
    /// Creates a new render target from the given size.
    pub fn new() -> MorphResult<Self> {
        let canvas = utils::canvas("morph_canvas")?;

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Ok(RenderTarget { canvas, context })
    }
}

impl graphics::RenderTarget<RenderContext> for RenderTarget {
    fn size(&self) -> Size {
        Size::new(self.canvas.width(), self.canvas.height())
    }

    fn set_size(&mut self, size: impl Into<Size>) {
        let size = size.into();
        self.canvas.set_width(size.width());
        self.canvas.set_height(size.height());
    }

    fn context(&self) -> MorphResult<RenderContext> {
        RenderContext::new(self.size())
    }

    fn draw_to_screen(&mut self, render_context: RenderContext) {
        let canvas = render_context.canvas();
        self.context
            .draw_image_with_html_canvas_element(&canvas, 0.0, 0.0)
            .map_err(|_| {
                MorphError::Backend("RenderTarget::draw_to_screen: could not draw to canvas.")
            }); 
    }
}
