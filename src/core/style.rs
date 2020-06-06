use crate::{graphics::Color, core::State, geometry::Thickness};

#[derive(Debug, Default, Clone, Copy)]
pub struct Style {
    pub color: Option<Color>,
    pub background: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<u32>,
    pub padding: Option<Thickness>
}

pub trait IntoStyle {
    fn into_style(&self, state: Option<State>) -> Style;
}

pub trait BaseStyle : IntoStyle + Default {
    fn default_button() -> Self;
    fn primary_button() -> Self;
    fn default_label() -> Self;
}