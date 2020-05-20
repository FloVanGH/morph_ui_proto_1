use core::ops::{Add, Sub};

use derive_more::{Constructor, From, Into};

/// Represents a (pixel) position on the screen.
#[derive(Default, Copy, Clone, Debug, From, Into, PartialEq, Constructor)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    /// Gets the x coordinate.
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Sets the x coordinate.
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    /// Gets the y coordinate.
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Sets the y coordinate.
    pub fn set_y(&mut self, y: i32) {
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