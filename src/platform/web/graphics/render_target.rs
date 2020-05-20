use std::any::Any;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::{super::utils, RenderContext};
use crate::{geometry::Size, graphics, result::*};

/// The `RenderTarget` is used to draw the content of a `RenderContext` on the screen.
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

impl graphics::RenderTarget for RenderTarget {
    fn size(&self) -> Size {
        Size::new(self.canvas.width(), self.canvas.height())
    }

    fn set_size(&mut self, size: impl Into<Size>) {
        let size = size.into();
        self.canvas.set_width(size.width());
        self.canvas.set_height(size.height());

        let context = self
            .canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.begin_path();

        use std::f64;

        // Draw the outer circle.
        context
            .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the mouth.
        context.move_to(110.0, 75.0);
        context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

        // Draw the left eye.
        context.move_to(65.0, 65.0);
        context
            .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the right eye.
        context.move_to(95.0, 65.0);
        context
            .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        context.stroke();
    }

    fn context(&self) -> MorphResult<Box<dyn graphics::RenderContext>> {
        Ok(Box::new(RenderContext::new(self.size())?))
    }

    fn draw_to_screen(&mut self, render_context: impl Into<Box<dyn Any>>) {
        let render_context = render_context
            .into()
            .downcast::<RenderContext>()
            .map_err(|_| {
                MorphError::Backend("RenderTarget::draw_to_screen: Could downcast render context.")
            })?;
    }
}
