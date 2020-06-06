use crate::{
    embedded_graphics::{pixelcolor::Rgb565, DrawTarget,  mock_display::MockDisplay},
   
};

use crate::shell::Backend;

pub struct RaqoteBackend
{
    draw_target: MockDisplay<Rgb565>
}

impl RaqoteBackend
{
    pub fn new(draw_target: MockDisplay<Rgb565>) -> Self {
        RaqoteBackend { draw_target }
    }
}

impl Backend<MockDisplay<Rgb565>, Rgb565> for RaqoteBackend
{
    fn init(&mut self) {
       
    }
    fn draw_target(&mut self) -> &mut MockDisplay<Rgb565> {
        &mut self.draw_target
    }

    fn flush(&mut self) {
      
    }
}
