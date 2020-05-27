use embedded_graphics::{pixelcolor::Rgb565, DrawTarget, geometry::Size, drawable::Pixel};
use crate::result::*;

pub struct WasmDisplay {

}

impl WasmDisplay {
    pub fn new() -> Self {
        WasmDisplay {

        }
    }
}

impl DrawTarget<Rgb565> for WasmDisplay {
    type Error = MorphError;

    fn draw_pixel(&mut self, pixel: Pixel<Rgb565>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(64, 64)
    }
}
