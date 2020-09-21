use heapless::{consts::*, String};

use stretch::{
    geometry::Size,
    style::{Dimension, Style as LayoutStyle},
};

use crate::{
    core::{BaseStyle, Drawable, State, Widget},
    geometry::Thickness,
    result::*,
    theme::*,
};

#[derive(Debug, Clone)]
pub struct Button<Message, S> {
    on_tap: Option<Message>,
    text: &'static str,
    layout_style: LayoutStyle,
    style: S,
}

impl<Message, S> Default for Button<Message, S>
where
    S: BaseStyle,
{
    fn default() -> Self {
        let layout_style = LayoutStyle {
            size: Size {
                width: Dimension::Points(32.),
                height: Dimension::Points(32.),
            },
            // min_size: Size { width: Dimension::Points(32.), height: Dimension::Undefined },
            ..Default::default()
        };
        Button {
            on_tap: None,
            text: "",
            layout_style,
            style: S::default_button(),
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

    pub fn text(mut self, text: &'static str) -> Self {
        self.text = text;
        self
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
        widget.name = "Button";
        widget.state = Some(State {
            is_pressed: Some(false),
        });
        widget.text = Some(self.text);
        widget
            .drawables
            .push(Drawable::Rectangle)
            .map_err(|_| MorphError::OutOfBounds("Cannot add rectangle drawable to button."))?;
        widget
            .drawables
            .push(Drawable::Text)
            .map_err(|_| MorphError::OutOfBounds("Cannot add text drawable to button."))?;
        widget.layout_style = self.layout_style;
        Ok(widget)
    }
}
