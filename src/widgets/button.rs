use heapless::{consts::*, String};

use stretch::style::Style;

use crate::{
    core::{Drawable, Widget},
    geometry::Thickness,
    result::*,
};

#[derive(Debug, Clone)]
pub struct Button<Message> {
    on_tap: Option<Message>,
    text: String<U64>,
    layout_style: Style
}

impl<Message> Default for Button<Message> {
    fn default() -> Self {
        Button {
            on_tap: None,
            text: String::default(),
            layout_style: Style::default()
        }
    }
}

impl<Message> Button<Message> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: &str) -> MorphResult<Self> {
        self.text.clear();
        self.text.push_str(text).map_err(|_| MorphError::OutOfBounds("Could not set text to Label. Text is to long."))?;
        Ok(self)
    }

    pub fn on_tap(mut self, message: Message) -> Self {
        self.on_tap = Some(message);
        self
    }

    pub fn margin(mut self, margin: impl Into<Thickness>) -> Self {
        self.layout_style.margin = margin.into().into();
        self
    }
}

impl<Message> IntoResult<Widget<Message>> for Button<Message>
{
    fn into_result(self) -> MorphResult<Widget<Message>> {
        let mut widget = Widget::new()?;
        widget.text = Some(self.text);
        widget
            .drawables
            .push(Drawable::Rectangle)
            .map_err(|_| MorphError::OutOfBounds("Could not add rectangle drawable to button."))?;
        widget
            .drawables
            .push(Drawable::Text)
            .map_err(|_| MorphError::OutOfBounds("Could not add text drawable to button."))?;
        widget.layout_style = self.layout_style;
        Ok(widget)
    }
}
