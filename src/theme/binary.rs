use heapless::{String, consts::*};
use crate::{core::{Style, IntoStyle, BaseStyle, State}, graphics::Color};

use super::colors::*;

pub enum BinaryTheme {
    PrimaryButton
}

impl Default for BinaryTheme {
    fn default() -> Self {
        BinaryTheme::PrimaryButton
    }
}

impl BaseStyle for BinaryTheme {
    fn primary_button() -> Self {
        BinaryTheme::PrimaryButton
    }
}

fn primary_button_style(state: &State) -> Style {
    let mut style = Style::default();
   
    style.border_color = Some(Color::from(WHITE_COLOR));
    style.background = Some(Color::from(BLACK_COLOR));
    style.color = Some(Color::from(WHITE_COLOR));

    if let Some(is_pressed) = state.is_pressed {
        if is_pressed {
            style.border_width = Some(2);
            style.background = Some(Color::from(BLACK_COLOR));
        } else {
            style.border_width = Some(1);
        }      
    }

    style
}

impl IntoStyle for BinaryTheme {
    fn into_style(&self, state: &State) -> Style {
        match self {
            BinaryTheme::PrimaryButton => primary_button_style(state)
        }
    }   
}