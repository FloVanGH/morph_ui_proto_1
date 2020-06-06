use heapless::{String, consts::*};
use crate::{core::{Style, IntoStyle, BaseStyle, State}, graphics::Color};


pub use colors::*;

pub mod binary;
mod colors;

pub enum Theme {
    PrimaryButton
}

impl Default for Theme {
    fn default() -> Self {
        Theme::PrimaryButton
    }
}

impl BaseStyle for Theme {
    fn primary_button() -> Self {
        Theme::PrimaryButton
    }
}

fn primary_button_style(state: &State) -> Style {
    let mut style = Style::default();

    crate::log("blub style");

    style.border_width = Some(2);
    style.border_color = Some(Color::from(WHITE_COLOR));

    if let Some(is_pressed) = state.is_pressed {
        if is_pressed {
            style.background = Some(Color::from(CYAN_COLOR));
            style.color = Some(Color::from(WHITE_COLOR));
        } else {
            style.background = Some(Color::from(BLACK_COLOR));
            style.color = Some(Color::from(WHITE_COLOR));
        }      
    }

    style
}

impl IntoStyle for Theme {
    fn into_style(&self, state: &State) -> Style {
        match self {
            Theme::PrimaryButton => primary_button_style(state)
        }
    }   
}