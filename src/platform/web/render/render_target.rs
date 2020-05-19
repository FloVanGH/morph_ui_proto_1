use crate::geometry::Size;
use super::RenderContext;

/// The `RenderTarget` is used to draw the content of a `RenderContext` on the screen.
pub struct RenderTarget {
    size: Size
}

impl RenderTarget {
    /// Creates a new render target from the given size.
    pub fn new(size: Size) -> Self {
        RenderTarget {
            size
        }
    }

    /// Draw the given `RenderContext` on the screen.
    pub fn draw_to_screen(&mut self, render_context: RenderContext) {

    }
}