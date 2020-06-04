

use heapless::{consts::*, String};

use crate::{core::{Widget, Drawable}, result::*, embedded_graphics::{geometry::Point, pixelcolor::PixelColor, fonts::Text }};

#[derive(Debug, Clone)]
pub struct Label {
    text: String<U16>
}

impl Default for Label {
    fn default() -> Self {
        Label {
            text: String::default()
        }
    }
}

impl Label {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: impl Into<String<U16>>) -> Self {
        self.text = text.into();
        self
    }
}

impl<Message, C> IntoResult<Widget<Message, C>> for Label where C: PixelColor + From<<C as PixelColor>::Raw> {
    fn into_result(self) -> MorphResult<Widget<Message, C>> {
        let mut widget = Widget::new()?;
        widget.text = Some(self.text);
        widget
        .drawables
        .push(Drawable::Text(Text::new("", Point::default())))
        .map_err(|_| MorphError::OutOfBounds("Could not add text drawable to label."))?;
        Ok(widget)
    }  
}