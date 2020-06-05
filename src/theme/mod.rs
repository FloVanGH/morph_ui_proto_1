
use crate::{core::{Style, IntoStyle}, graphics::Color};

pub struct PrimaryButtonStyle;

impl<Message> IntoStyle<Message> for PrimaryButtonStyle {
    fn into_style(self, widget: &crate::core::Widget<Message>) -> Style {
        let mut style = Style::default();

        if let Some(is_pressed) = widget.is_pressed {
            style.background = Some(Color::from("#000000"));
        }

        style
    }
    
}

pub struct DefaultTheme {

}