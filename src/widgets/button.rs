use heapless::{consts::*, String};

use crate::{
    core::{Drawable, Widget},
    embedded_graphics::{fonts::Text, geometry::Point, pixelcolor::PixelColor, primitives::*},
    result::*,
};

#[derive(Debug, Clone)]
pub struct Button<Message> {
    on_tap: Option<Message>,
    text: String<U16>,
}

impl<Message> Default for Button<Message> {
    fn default() -> Self {
        Button {
            on_tap: None,
            text: String::default(),
        }
    }
}

impl<Message> Button<Message> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: impl Into<String<U16>>) -> Self {
        self.text = text.into();
        self
    }

    pub fn on_tap(mut self, message: Message) -> Self {
        self.on_tap = Some(message);
        self
    }
}

impl<Message, C> IntoResult<Widget<Message, C>> for Button<Message>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn into_result(self) -> MorphResult<Widget<Message, C>> {
        let mut widget = Widget::new()?;
        widget.text = Some(self.text);
        widget
            .drawables
            .push(Drawable::Rectangle(Rectangle::default()))
            .map_err(|_| MorphError::OutOfBounds("Could not add rectangle drawable to button."))?;
        widget
            .drawables
            .push(Drawable::Text(Text::new("", Point::default())))
            .map_err(|_| MorphError::OutOfBounds("Could not add text drawable to button."))?;
        Ok(widget)
    }
}
