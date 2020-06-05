use crate::{graphics::Color, core::Widget};

#[derive(Debug, Default, Clone)]
pub struct Style {
    pub color: Option<Color>,
    pub background: Option<Color>
}

pub trait IntoStyle<Message> {
    fn into_style(self, widget: &Widget<Message>) -> Style;
}