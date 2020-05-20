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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        let left = Point::new(5, 5);
        let right = Point::new(2, 3);
        assert_eq!(Point::new(3, 2), left - right);

        let left = Point::new(0, 0);
        let right = Point::new(2, 3);
        assert_eq!(Point::new(-2, -3), left - right);

        let left = Point::new(-2, -3);
        let right = Point::new(2, -3);
        assert_eq!(Point::new(-4, 0), left - right);
    }

    #[test]
    fn test_add() {
        let left = Point::new(5, 5);
        let right = Point::new(2, 3);
        assert_eq!(Point::new(7, 8), left + right);

        let left = Point::new(0, 0);
        let right = Point::new(2, 3);
        assert_eq!(Point::new(2, 3), left + right);

        let left = Point::new(-2, -3);
        let right = Point::new(2, -3);
        assert_eq!(Point::new(0, -6), left + right);
    }
}