use std::any::Any;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::super::utils;
use crate::{geometry::*, graphics, result::*};

/// The `RenderContext` provides different draw methods.
#[derive(Clone, Debug, PartialEq)]
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

        Ok(RenderContext { canvas, context })
    }

    /// Consumes the canvas of the render context.
    pub fn canvas(self) -> web_sys::HtmlCanvasElement {
        self.canvas
    }
}

impl graphics::RenderContext for RenderContext {
    fn begin_path(&mut self) {
        self.context.begin_path();
    }

    fn close_path(&mut self) {
        self.context.close_path();
    }

    fn save(&mut self) {
        self.context.save();
    }

    fn restore(&mut self) {
        self.context.restore();
    }

    fn set_stroke_style(&mut self) {
        todo!()
    }

    fn set_fill_style(&mut self) {
        todo!()
    }

    fn set_line_width(&mut self, width: u32) {
        self.context.set_line_width(width as f64);
    }

    fn move_to(&mut self, position: impl Into<Point>) {
        let position = position.into();
        self.context
            .move_to(position.x() as f64, position.y() as f64);
    }

    fn line_to(&mut self, position: impl Into<Point>) {
        let position = position.into();
        self.context
            .line_to(position.x() as f64, position.y() as f64);
    }

    fn fill_rect(&mut self, position: impl Into<Point>, size: impl Into<Size>) {
        let position = position.into();
        let size = size.into();
        self.context.fill_rect(
            position.x() as f64,
            position.y() as f64,
            size.width() as f64,
            size.height() as f64,
        );
    }

    fn stroke_rect(&mut self, position: impl Into<Point>, size: impl Into<Size>) {
        let position = position.into();
        let size = size.into();
        self.context.fill_rect(
            position.x() as f64,
            position.y() as f64,
            size.width() as f64,
            size.height() as f64,
        );
    }

    fn fill_triangle(&mut self, position: impl Into<Point>, size: impl Into<Size>) {
        todo!()
    }

    fn stroke_triangle(&mut self, position: impl Into<Point>, size: impl Into<Size>) {
        todo!()
    }

    fn fill_circle(&mut self, position: impl Into<Point>, size: impl Into<Size>) {
        todo!()
    }

    fn stroke_circle(&mut self, position: impl Into<Point>, size: impl Into<Size>) {
        todo!()
    }

    fn draw_image(&mut self) {
        todo!()
    }

    fn set_font_size(&mut self, size: u32) {
        todo!()
    }

    fn fill_text(&mut self, position: impl Into<Point>, text: &str) {
        todo!()
    }
}
