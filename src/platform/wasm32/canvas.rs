use crate::{
    canvas_display::CanvasDisplay,
    embedded_graphics::{pixelcolor::Rgb565, DrawTarget,  mock_display::MockDisplay},
};

use crate::shell::Backend;

pub struct CanvasBackend
{
    draw_target: CanvasDisplay
}

impl CanvasBackend
{
    pub fn new(draw_target: CanvasDisplay) -> Self {
        CanvasBackend { draw_target }
    }
}

impl Backend<CanvasDisplay, Rgb565> for CanvasBackend
{
    fn init(&mut self) {
       
    }
    fn draw_target(&mut self) -> &mut CanvasDisplay {
        &mut self.draw_target
    }

    fn flush(&mut self) {
        self.draw_target.flip().unwrap();
    }
}
