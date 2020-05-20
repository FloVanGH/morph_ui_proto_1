use std::any::Any;

use crate::{geometry::Size, result::*};

use super::RenderContext;

pub trait RenderTarget<C> where C: RenderContext {
     /// Gets the size of the render target on the screen.
    fn size(&self) -> Size;

    /// Sets the size of the render target on the screen.
    fn set_size(&mut self, size: impl Into<Size>);

    /// Creates an get the render context.
    fn context(&self) -> MorphResult<C>;

    /// Draw the given `RenderContext` on the screen.
    fn draw_to_screen(&mut self, render_context: C);
}