use embedded_graphics::{
    fonts::Text,
    image::{Image, ImageRaw},
    pixelcolor::{raw::LittleEndian, PixelColor},
    primitives::*,
};

pub enum Drawable<C: 'static>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    Rectangle(Rectangle),
    Line(Line),
    Circle(Circle),
    Triangle(Triangle),
    Text(Text<'static>),
    Image(Image<'static, ImageRaw<'static, C, LittleEndian>, C>),
}
