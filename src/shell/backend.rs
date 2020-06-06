use crate::embedded_graphics::{DrawTarget, pixelcolor::PixelColor};

pub trait Backend<D: DrawTarget<C> + 'static, C: 'static> where C: PixelColor + From<<C as PixelColor>::Raw>,  {

    fn init(&mut self);
    
    fn draw_target(&mut self) -> &mut D;

    fn flush(&mut self);
}