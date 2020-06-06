use crate::embedded_graphics::{DrawTarget, pixelcolor::BinaryColor};

use sh1106::{prelude::*, Builder, interface::DisplayInterface};
use crate::shell::Backend;

pub struct Sh1106Backend<DI: 'static> where DI: DisplayInterface {
    draw_target: GraphicsMode<DI>
}

impl<DI> Sh1106Backend<DI> where DI: DisplayInterface {
    pub fn new(draw_target: GraphicsMode<DI>) -> Self {
        Sh1106Backend {
            draw_target
        }
    }
}

impl<DI> Backend<GraphicsMode<DI>, BinaryColor> for Sh1106Backend<DI> where DI: DisplayInterface {
    fn init(&mut self) {
        self.draw_target.init();
        self.draw_target.flush();
    }
    fn draw_target(&mut self) -> &mut GraphicsMode<DI> {
        &mut self.draw_target
    }

    fn flush(&mut self) {
        self.draw_target.flush();
    }
}