
use tinybmp::{Bmp, BmpIterator};
use crate::{geometry::*, result::*};

use super::Color;

/// Represents an image object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Image<'a> {
    data: Bmp<'a>,
}

impl<'a> Image<'a> {
    /// Creates a new image from a bmp image.
    pub fn from_bmp(bmp: Bmp<'a>) -> MorphResult<Self> {
        Ok(Image {
            data: bmp
        })
    }

    /// Gets the width of the image.
    pub fn width(&self) -> u32 {
        self.data.width()
    }

    /// Gets the height of the image.
    pub fn height(&self) -> u32 {
        self.data.height()
    }

    pub fn data(&self) -> &Bmp<'a> {
        &self.data
    }
}

impl<'a> IntoIterator for &'a Image<'a> {
    type Item = Color;
    type IntoIter = ImageIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ImageIterator {
            iterator: self.data.into_iter()
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ImageIterator<'a> {
    iterator: BmpIterator<'a>
}

impl<'a> Iterator for ImageIterator<'a> {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pixel) = self.iterator.next() {
            return Some(Color { data: pixel.color} );
        }

        None
    }
}