use heapless::{consts::*, String, Vec};
use stretch::style::Style;

use crate::{
    core::{Drawable, Widget},
    geometry::Thickness,
    embedded_graphics::geometry::Size,
    result::*,
};

#[derive(Debug, Clone)]
pub struct Image {
    layout_style: Style,
    size: Size,
    data: &'static [u8]
}

impl Image {
    pub fn new(data: &'static [u8], width: u32, height: u32) -> MorphResult<Self> {
        Ok(Image {
            layout_style: Style::default(),
            data,
            size: Size::new(width, height)
        })
    }


    pub fn margin(mut self, margin: impl Into<Thickness>) -> Self {
        self.layout_style.margin = margin.into().into();
        self
    }
}

impl<Message> IntoResult<Widget<Message>> for Image
{
    fn into_result(self) -> MorphResult<Widget<Message>> {
        let mut widget = Widget::new()?;
        widget.size = self.size;
        widget.name.push_str("Image").map_err(|_| MorphError::OutOfBounds("Could not set name for label."))?;
        widget.image = Some(self.data);
        widget
            .drawables
            .push(Drawable::Image)
            .map_err(|_| MorphError::OutOfBounds("Could not add text drawable to label."))?;
        widget.layout_style = self.layout_style;
        Ok(widget)
    }
}
