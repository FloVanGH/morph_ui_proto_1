use super::Color;

/// A `Brush`describes how a shape is filled or stroked.
pub enum Brush {
    /// Paints an area with a solid color.
    Color(Color)
}

impl From<Brush> for Color {
    fn from(b: Brush) -> Color {
        match b {
            Brush::Color(color) => color,
            _ => Color::rgb(0, 0, 0),
        }
    }
}

impl Default for Brush {
    fn default() -> Self {
        Brush::Color(Color::rgba(0, 0, 0, 0))
    }
}

impl From<Color> for Brush {
    fn from(c: Color) -> Brush {
        Brush::Color(c)
    }
}

impl From<&str> for Brush {
    fn from(s: &str) -> Brush {
        Brush::Color(Color::from(s))
    }
}