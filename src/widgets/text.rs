use heapless::{consts::*, String};
use stretch::style::Style;

use crate::{
    core::{Drawable, Widget},
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

impl<Message> IntoResult<Widget<Message>> for Label
{
    fn into_result(self) -> MorphResult<Widget<Message>> {
        let mut widget = Widget::new()?;
        widget.text = Some(self.text);
        widget
            .drawables
            .push(Drawable::Text)
            .map_err(|_| MorphError::OutOfBounds("Could not add text drawable to label."))?;
        widget.layout_style = self.layout_style;
        Ok(widget)
    }
}
