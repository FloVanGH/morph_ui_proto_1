use heapless::{consts::*, String};
use stretch::style::Style;

use crate::{
    core::{Drawable, Widget},
    embedded_graphics::{fonts::Text, geometry::Point, pixelcolor::PixelColor},
    geometry::Thickness,
    result::*,
};

#[derive(Debug, Clone)]
pub struct Label {
    text: String<U64>,
    layout_style: Style,
}

impl Default for Label {
    fn default() -> Self {
        Label {
            text: String::default(),
            layout_style: Style::default(),
        }
    }
}

impl Label {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: &str) -> MorphResult<Self> {
        self.text.clear();
        self.text.push_str(text).map_err(|_| MorphError::OutOfBounds("Could not set text to Label. Text is to long."))?;
        Ok(self)
    }

    pub fn margin(mut self, margin: impl Into<Thickness>) -> Self {
        self.layout_style.margin = margin.into().into();
        self
    }
}

impl<Message, C> IntoResult<Widget<Message, C>> for Label
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    fn into_result(self) -> MorphResult<Widget<Message, C>> {
        let mut widget = Widget::new()?;
        widget.text = Some(self.text);
        widget
            .drawables
            .push(Drawable::Text(Text::new("", Point::default())))
            .map_err(|_| MorphError::OutOfBounds("Could not add text drawable to label."))?;
        widget.layout_style = self.layout_style;
        Ok(widget)
    }
}
