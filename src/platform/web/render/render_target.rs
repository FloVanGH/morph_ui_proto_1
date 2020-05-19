use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use super::{super::utils, RenderContext};
use crate::{geometry::Size, result::*};

/// The `RenderTarget` is used to draw the content of a `RenderContext` on the screen.
pub struct RenderTarget {
    size: Size,
    canvas: HtmlCanvasElement,
}

impl RenderTarget {
    /// Creates a new render target from the given size.
    pub fn new(size: Size) -> MorphResult<Self> {
        let canvas = utils::canvas("morph_canvas")?;
        canvas.set_width(size.width());
        canvas.set_height(size.height());

        let context = canvas
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

        Ok(RenderTarget { size, canvas })
    }

    /// Draw the given `RenderContext` on the screen.
    pub fn draw_to_screen(&mut self, render_context: RenderContext) {}
}
