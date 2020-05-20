use std::any::Any;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::super::utils;
use crate::{geometry::Size, graphics, result::*};

/// The `RenderContext` provides different draw methods.
pub struct RenderContext {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

impl RenderContext {
    /// Creates a new render context from the given size.
    pub fn new(size: impl Into<Size>) -> MorphResult<Self> {
        let size = size.into();

        let canvas = utils::create_canvas()?;
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

        Ok(RenderContext { canvas, context })
    }

    /// Consumes the canvas of the render context.
    pub fn canvas(self) -> web_sys::HtmlCanvasElement {
        self.canvas
    }
}

impl graphics::RenderContext for RenderContext {}
