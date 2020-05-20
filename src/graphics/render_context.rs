use crate::geometry::*;

/// Provides the 2D rendering context .
pub trait RenderContext {
    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    fn begin_path(&mut self);

    /// Attempts to add a straight line from the current position to the start of the current sub-path. If the shape has already been closed or has only one position, this function does nothing.
    fn close_path(&mut self);

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    fn save(&mut self);

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    fn restore(&mut self);

    /// Specifies the fill stroke to use inside shapes.
    fn set_stroke_style(&mut self);

    /// Specifies the fill color to use inside shapes.
    fn set_fill_style(&mut self);

    /// Sets the thickness of lines.
    fn set_line_width(&mut self, width: u32);

    /// Begins a new sub-path at the position specified by the given {x, y} coordinates.
    fn move_to(&mut self, position: impl Into<Point>);

    /// Adds a straight line to the current sub-path by connecting the sub-path's last position to the specified {x, y} coordinates.
    fn line_to(&mut self, position: impl Into<Point>);

    /// Draws a rectangle that is filled according to the current fill_style
    fn fill_rect(&mut self, position: impl Into<Point>, size: impl Into<Size>);

    /// Draws a rectangle that is stroked (outlined) according to the current stroke_style;
    fn stroke_rect(&mut self, position: impl Into<Point>, size: impl Into<Size>);

    /// Draws a triangle that is filled according to the current fill_style
    fn fill_triangle(&mut self, position: impl Into<Point>, size: impl Into<Size>);

    /// Draws a triangle that is stroked (outlined) according to the current stroke_style;
    fn stroke_triangle(&mut self, position: impl Into<Point>, size: impl Into<Size>);

    /// Draws a circle that is filled according to the current fill_style
    fn fill_circle(&mut self, position: impl Into<Point>, size: impl Into<Size>);

    /// Draws a circle that is stroked (outlined) according to the current stroke_style;
    fn stroke_circle(&mut self, position: impl Into<Point>, size: impl Into<Size>);

    /// Draws an image.
    fn draw_image(&mut self);

    /// Specifies the font size.
    fn set_font_size(&mut self, size: u32);

    /// Draws (fills) a given text at the given (x, y) position.
    fn fill_text(&mut self, position: impl Into<Point>, text: &str);
}