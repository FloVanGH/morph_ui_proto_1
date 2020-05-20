use crate::{geometry::*, graphics, result::*};

/// The `RenderContext` provides different draw methods. This implementation is based on the web_sys crate and is used to draw an a browser canvas.
#[derive(Clone, Debug, PartialEq)]
pub struct RenderContext {
}

impl graphics::RenderContext for RenderContext {
    fn begin_path(&mut self) {
        todo!()
    }
    fn close_path(&mut self) {
        todo!()
    }
    fn save(&mut self) {
        todo!()
    }
    fn restore(&mut self) {
        todo!()
    }
    fn set_stroke_style(&mut self) {
        todo!()
    }
    fn set_fill_style(&mut self) {
        todo!()
    }
    fn set_line_width(&mut self, width: u32) {
        todo!()
    }
    fn move_to(&mut self, position: impl Into<Point>) {
        todo!()
    }
    fn line_to(&mut self, position: impl Into<Point>) {
        todo!()
    }
    fn fill_rect(&mut self, position: impl Into<Point>, size: impl Into<Size>) {
        todo!()
    }
    fn stroke_rect(&mut self, position: impl Into<Point>, size: impl Into<Size>) {
        todo!()
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