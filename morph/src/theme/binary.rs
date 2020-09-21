use crate::{
    core::{BaseStyle, IntoStyle, State, Style},
    graphics::Color,
};
use heapless::{consts::*, String};

use super::colors::*;

pub enum BinaryTheme {
    DefaultButton,
    PrimaryButton,
    DefaultLabel,
}

impl Default for BinaryTheme {
    fn default() -> Self {
        BinaryTheme::DefaultLabel
    }
}

impl BaseStyle for BinaryTheme {
    fn default_button() -> Self {
        BinaryTheme::DefaultButton
    }

    fn primary_button() -> Self {
        BinaryTheme::PrimaryButton
    }

    fn default_label() -> Self {
        BinaryTheme::DefaultLabel
    }
}

fn default_label() -> Style {
    let mut style = Style::default();
    style.color = Some(Color::from(WHITE_COLOR));
    style
}

fn default_button(state: Option<State>) -> Style {
    let mut style = Style::default();

    style.border_color = Some(Color::from(WHITE_COLOR));
    style.background = Some(Color::from(WHITE_COLOR));
    style.color = Some(Color::from(WHITE_COLOR));

    if let Some(state) = state {
        if let Some(is_pressed) = state.is_pressed {
            if is_pressed {
                style.border_width = Some(2);
                style.background = Some(Color::from(WHITE_COLOR));
            } else {
                style.border_width = Some(1);
            }
        }
    }

    style
}

fn primary_button(state: Option<State>) -> Style {
    let mut style = Style::default();

    if let Some(state) = state {
        if let Some(is_pressed) = state.is_pressed {
            if is_pressed {
                style.border_color = Some(Color::from(WHITE_COLOR));
                style.border_width = Some(2);
                style.background = Some(Color::from(WHITE_COLOR));
                style.color = Some(Color::from(WHITE_COLOR));
            } else {
                style.background = Some(Color::from(WHITE_COLOR));
                style.color = Some(Color::from(WHITE_COLOR));
            }
        }
    }

    style
}

impl IntoStyle for BinaryTheme {
    fn into_style(&self, state: Option<State>) -> Style {
        match self {
            BinaryTheme::PrimaryButton => primary_button(state),
            BinaryTheme::DefaultButton => default_button(state),
            BinaryTheme::DefaultLabel => default_label(),
        }
    }
}
