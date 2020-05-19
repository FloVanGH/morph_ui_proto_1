use core::ops::{Add, Sub};

use derive_more::{Constructor, From, Into};

/// Represents a (pixel) position on the screen.
#[derive(Default, Copy, Clone, Debug, From, Into, PartialEq, Constructor)]
pub struct Point {
    x: u32,
    y: u32
}

impl Point {
    /// Gets the x coordinate.
    pub fn x(&self) -> u32 {
        self.x
    }

    /// Sets the x coordinate.
    pub fn set_x(&mut self, x: u32) {
        self.x = x;
    }

    /// Gets the y coordinate.
    pub fn y(&self) -> u32 {
        self.y
    }

    /// Sets the y coordinate.
    pub fn set_y(&mut self, y: u32) {
        self.y = y;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}