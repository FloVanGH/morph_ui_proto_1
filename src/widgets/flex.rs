use heapless::{consts::*, Vec};
use stretch::style::Style;

use crate::{core::Widget, geometry::Thickness, embedded_graphics::pixelcolor::PixelColor, result::*};

pub struct Flex<Message, C: 'static>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    children: Vec<Widget<Message, C>, U8>,
    layout_style: Style,
}

impl<Message, C> Flex<Message, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Message, C> Default for Flex<Message, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn default() -> Self {
        Flex {
            children: Vec::default(),
            layout_style: Style::default(),
        }
    }
}

impl<Message, C> Flex<Message, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    pub fn child(mut self, child: impl IntoResult<Widget<Message, C>>) -> MorphResult<Self> {
        self.children
            .push(child.into_result()?)
            .map_err(|_| MorphError::OutOfBounds("Flex::child: could not add more children."))?;
        Ok(self)
    }

    pub fn margin(mut self, margin: impl Into<Thickness>) -> Self {
        self.layout_style.margin = margin.into().into();
        self
    }
}

impl<Message, C> IntoResult<Widget<Message, C>> for Flex<Message, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn into_result(self) -> MorphResult<Widget<Message, C>> {
        Widget::new()
    }
}
