use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::super::utils;
use crate::{geometry::*, graphics, result::*};

/// The `RenderContext` provides different draw methods. This implementation is based on the web_sys crate and is used to draw an a browser canvas.
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

    fn circle(&mut self, center: impl Into<Point>, radius: u32) {
        let center = center.into();
        self.context.arc(center.x() as f64, center.y() as f64, radius as f64, 0.0, 2.0 * std::f64::consts::PI);
    }

    fn triangle(&mut self, position_one: impl Into<Point>,  position_two: impl Into<Point>, position_three: impl Into<Point>) {
        let position_one = position_one.into();
        let position_two = position_two.into();
        let position_three = position_three.into();

        self.context.move_to(position_one.x() as f64, position_one.y() as f64);
        self.context.line_to(position_two.x() as f64, position_two.y() as f64);
        self.context.line_to(position_three.x() as f64, position_three.y() as f64);
        self.context.line_to(position_one.x() as f64, position_one.y() as f64);
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

    fn set_stroke_style(&mut self, brush: impl Into<graphics::Brush>) {
        self.context.set_stroke_style(&brush_to_js_value(brush.into()));
    }

    fn set_fill_style(&mut self, brush: impl Into<graphics::Brush>) {
        self.context.set_fill_style(&brush_to_js_value(brush.into()));
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
        self.context.stroke_rect(
            position.x() as f64,
            position.y() as f64,
            size.width() as f64,
            size.height() as f64,
        );
    }

    fn fill_triangle(&mut self, position_one: impl Into<Point>,  position_two: impl Into<Point>, position_three: impl Into<Point>) {
        self.begin_path();
        self.triangle(position_one, position_two, position_three);
        self.fill();
        self.close_path();
    }

    fn stroke_triangle(&mut self, position_one: impl Into<Point>,  position_two: impl Into<Point>, position_three: impl Into<Point>) {
        self.begin_path();
        self.triangle(position_one, position_two, position_three);
        self.stroke();
        self.close_path();
    }

    fn fill_circle(&mut self, center: impl Into<Point>, radius: u32) {
        self.begin_path();
        self.circle(center, radius);
        self.fill();
        self.close_path();
    }

    fn stroke_circle(&mut self, center: impl Into<Point>, radius: u32) {
        self.begin_path();
        self.circle(center, radius);
        self.stroke();
        self.close_path();
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

    fn fill(&mut self) {
        self.context.fill();
    }

    fn stroke(&mut self) {
        self.context.stroke();
    }  
}

fn brush_to_js_value(brush: graphics::Brush) -> JsValue {
    let color = {
        match brush {
            graphics::Brush::Color(color) => color,
        }
    };

    let mut color = format!("#{:x}", color.data);
    color.remove(1);
    color.remove(1);

    JsValue::from_str(color.as_str())
}
