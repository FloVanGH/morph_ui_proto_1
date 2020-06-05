use core::marker::PhantomData;

use heapless::{consts::*, String};
use stretch::style::Style as LayoutStyle;

use crate::{
    core::{Drawable, BaseStyle, Widget},
    geometry::Thickness,
    result::*,
};

#[derive(Debug, Clone)]
pub struct Label<Message, S> {
    text: String<U64>,
    layout_style: LayoutStyle,
    style: S,
    _phantom: PhantomData<Message>
}

impl<Message, S> Default for Label<Message, S> where S: BaseStyle  {
    fn default() -> Self {
        Label {
            text: String::default(),
            layout_style: LayoutStyle::default(),
            style: S::default(),
            _phantom: PhantomData::default()
        }
    }
}

impl<Message, S> Label<Message, S>  where S: BaseStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: impl Into<String<U64>>) -> MorphResult<Self> {
        self.text.clear();
        self.text.push_str(text.into().as_str()).map_err(|_| MorphError::OutOfBounds("Could not set text to label. Text is to long."))?;
        Ok(self)
    }

    pub fn margin(mut self, margin: impl Into<Thickness>) -> Self {
        self.layout_style.margin = margin.into().into();
        self
    }
}

impl<Message, S> IntoResult<Widget<Message, S>> for Label<Message, S> where S: BaseStyle
{
    fn into_result(self) -> MorphResult<Widget<Message, S>> {
        let mut widget = Widget::new()?;
        widget.name.push_str("Label").map_err(|_| MorphError::OutOfBounds("Could not set name for label."))?;
        widget.text = Some(self.text);
        widget
            .drawables
            .push(Drawable::Text)
            .map_err(|_| MorphError::OutOfBounds("Could not add text drawable to label."))?;
        widget.layout_style = self.layout_style;
        Ok(widget)
    }
}
