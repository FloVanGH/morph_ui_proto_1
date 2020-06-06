use heapless::{consts::*, String, Vec};
use stretch::{
    geometry::Size as LayoutSize,
    style::{Dimension, Style},
};

use crate::{
    core::{Drawable, Widget, BaseStyle},
    embedded_graphics::geometry::Size,
    geometry::Thickness,
    result::*,
};

#[derive(Debug, Clone)]
pub struct Image {
    layout_style: Style,
    size: Size,
    data: &'static [u8],
}

impl Image {
    pub fn new(data: &'static [u8], width: u32, height: u32) -> MorphResult<Self> {
        let mut style = Style::default();
        style.size = LayoutSize {
            width: Dimension::Points(width as f32),
            height: Dimension::Points(height as f32),
        };
        Ok(Image {
            layout_style: style,
            data,
            size: Size::new(width, height),
        })
    }

    pub fn margin(mut self, margin: impl Into<Thickness>) -> Self {
        self.layout_style.margin = margin.into().into();
        self
    }
}

impl<Message, S> IntoResult<Widget<Message, S>> for Image where S: BaseStyle {
    fn into_result(self) -> MorphResult<Widget<Message, S>> {
        let mut widget = Widget::new()?;
        widget.size = self.size;
        widget.name = "Image";
        widget.image = Some(self.data);
        widget
            .drawables
            .push(Drawable::Image)
            .map_err(|_| MorphError::OutOfBounds("Could not add text drawable to label."))?;
        // widget.layout_style = self.layout_style;
        Ok(widget)
    }
}
