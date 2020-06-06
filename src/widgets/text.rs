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
    text: &'static str,
    layout_style: LayoutStyle,
    style: S,
    _phantom: PhantomData<Message>
}

impl<Message, S> Default for Label<Message, S> where S: BaseStyle  {
    fn default() -> Self {
        Label {
            text: "",
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

    pub fn text(mut self, text: impl Into<&'static str>) -> Self {
        self.text = text.into();
        self
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
        widget.text = Some(self.text);
        widget.name = "Label";
        // widget.text = Some(self.text);
        widget
            .drawables
            .push(Drawable::Text)
            .map_err(|_| MorphError::OutOfBounds("Could not add text drawable to label."))?;
        // widget.layout_style = self.layout_style;
        Ok(widget)
    }
}
