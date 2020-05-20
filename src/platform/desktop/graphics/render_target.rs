use super::RenderContext;
use crate::{geometry::*, graphics, result::*};

/// The `RenderTarget` is used to draw the content of a `RenderContext` on the screen.
#[derive(Clone, Debug, PartialEq)]
pub struct RenderTarget {
   
}

impl RenderTarget {
    /// Creates a new render target from the given size.
    pub fn new() -> MorphResult<Self> {
       

        Ok(RenderTarget {  })
    }
}

impl graphics::RenderTarget<RenderContext> for RenderTarget {
    fn size(&self) -> Size {
        todo!()
    }
    fn set_size(&mut self, size: impl Into<Size>) {
        todo!()
    }
    fn context(&self) -> MorphResult<RenderContext> {
        todo!()
    }
    fn draw_to_screen(&mut self, render_context: RenderContext) {
        todo!()
    }
}