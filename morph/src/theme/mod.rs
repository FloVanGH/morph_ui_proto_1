use crate::{
    core::{BaseStyle, IntoStyle, State, Style},
    graphics::Color,
};
use heapless::{consts::*, String};

pub use colors::*;

pub mod binary;
mod colors;

pub enum Theme {
    DefaultButton,
    PrimaryButton,
    DefaultLabel,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::DefaultLabel
    }
}

impl BaseStyle for Theme {
    fn default_button() -> Self {
        Theme::DefaultButton
    }

    fn primary_button() -> Self {
        Theme::PrimaryButton
    }

    fn default_label() -> Self {
        Theme::DefaultLabel
    }
}

fn default_label() -> Style {
    let mut style = Style::default();
    style.color = Some(Color::from(WHITE_COLOR));
    style
}

fn default_button(state: Option<State>) -> Style {
    let mut style = Style::default();

    style.border_width = Some(2);
    style.border_color = Some(Color::from(WHITE_COLOR));

    if let Some(state) = state {
        if let Some(is_pressed) = state.is_pressed {
            if is_pressed {
                style.background = Some(Color::from(CYAN_COLOR));
                style.color = Some(Color::from(WHITE_COLOR));
            } else {
                style.background = Some(Color::from(WHITE_COLOR));
                style.color = Some(Color::from(WHITE_COLOR));
            }
        }
    }

    style
}

fn primary_button_style(state: Option<State>) -> Style {
    let mut style = Style::default();

    style.border_width = Some(2);
    style.border_color = Some(Color::from(CYAN_COLOR));

    if let Some(state) = state {
        if let Some(is_pressed) = state.is_pressed {
            if is_pressed {
                style.background = Some(Color::from(CYAN_COLOR));
                style.color = Some(Color::from(WHITE_COLOR));
            } else {
                style.background = Some(Color::from(BLACK_COLOR));
                style.color = Some(Color::from(CYAN_COLOR));
            }
        }
    }

    style
}

impl IntoStyle for Theme {
    fn into_style(&self, state: Option<State>) -> Style {
        match self {
            Theme::PrimaryButton => primary_button_style(state),
            Theme::DefaultButton => default_button(state),
            Theme::DefaultLabel => default_label(),
        }
    }
}
