/// A r g b a color.
#[derive(Default, Debug, Clone, Copy)]
pub struct Color {
    pub data: u32,
}

impl Color {
    /// Create a new color from RGB
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            data: 0xFF00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }

    /// Create a new color from RGB and alpha values
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            data: ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }

    /// Get the r value
    pub fn r(self) -> u8 {
        ((self.data & 0x00FF_0000) >> 16) as u8
    }

    /// Get the g value
    pub fn g(self) -> u8 {
        ((self.data & 0x0000_FF00) >> 8) as u8
    }

    /// Get the b value
    pub fn b(self) -> u8 {
        (self.data & 0x0000_00FF) as u8
    }

    /// Get the alpha value
    pub fn a(self) -> u8 {
        ((self.data & 0xFF00_0000) >> 24) as u8
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Color {
        let clean_hex = s.trim_start_matches('#');
        match clean_hex.len() {
            6 | 8 => {
                let mut x = match u32::from_str_radix(&clean_hex, 16) {
                    Ok(x) => x,
                    Err(_) => 0,
                };

                if clean_hex.len() == 6 {
                    x |= 0xFF_000_000;
                }

                Color { data: x }
            }
            _ => Color { data: 0 },
        }
    }
}

/// Compares two colors (the alpha value is ignored)
impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r() == other.r() && self.g() == other.g() && self.b() == other.b()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partial_eq() {
        assert_eq!(true, Color::rgb(1, 2, 3) == Color::rgba(1, 2, 3, 200));
        assert_eq!(false, Color::rgb(1, 2, 3) == Color::rgba(11, 2, 3, 200));
        assert_eq!(true, Color::rgba(1, 2, 3, 200) == Color::rgba(1, 2, 3, 200));
    }
}
