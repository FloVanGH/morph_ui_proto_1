use derive_more::{Constructor, From, Into};

/// Represents the (pixel) size of an visual object on the screen.
#[derive(Default, Debug, From, Into, PartialEq, Constructor)]
pub struct Size {
    width: u32,
    height: u32
}

impl Size {
    /// Gets the width.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Sets the width.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    /// Gets the height.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Sets the height.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}