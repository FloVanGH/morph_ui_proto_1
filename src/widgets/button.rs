use heapless::{consts::*, String};

use stretch::style::Style as LayoutStyle;

use crate::{
    core::{BaseStyle, Drawable, State, Widget},
    geometry::Thickness,
    result::*,
    theme::*,
};

#[derive(Debug, Clone)]
pub struct Button<Message, S> {
    on_tap: Option<Message>,
    text: String<U64>,
    layout_style: LayoutStyle,
    style: S,
}

impl<Message, S> Default for Button<Message, S>
where
    S: BaseStyle,
{
    fn default() -> Self {
        Button {
            on_tap: None,
            text: String::default(),
            layout_style: LayoutStyle::default(),
            style: S::primary_button(),
        }
    }
}

impl<Message, S> Button<Message, S>
where
    S: BaseStyle,
{
    pub fn from_style(style: S) -> Self {
        let mut button = Self::default();
        button.style = style;
        button
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: impl Into<String<U64>>) -> MorphResult<Self> {
        self.text.clear();
        self.text.push_str(text.into().as_str()).map_err(|_| {
            MorphError::OutOfBounds("Could not set text to Label. Text is to long.")
        })?;
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

impl<Message, S> IntoResult<Widget<Message, S>> for Button<Message, S>
where
    S: BaseStyle,
{
    fn into_result(self) -> MorphResult<Widget<Message, S>> {
        let mut widget = Widget::new()?;
        widget.style = Some(self.style);
        widget
            .name
            .push_str("Button")
            .map_err(|_| MorphError::OutOfBounds("Could not set name for button."))?;
        // widget.state = Some(State {
        //     is_pressed: Some(false),
        // });
        // widget.text = Some(self.text);
        widget
            .drawables
            .push(Drawable::Rectangle)
            .map_err(|_| MorphError::OutOfBounds("Could not add rectangle drawable to button."))?;
        widget
            .drawables
            .push(Drawable::Text)
            .map_err(|_| MorphError::OutOfBounds("Could not add text drawable to button."))?;
        // widget.layout_style = self.layout_style;
        Ok(widget)
    }
}
