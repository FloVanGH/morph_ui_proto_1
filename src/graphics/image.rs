
use tinybmp::Bmp;
use crate::{geometry::*, result::*};

/// Represents an image object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Image<'a> {
    data: Bmp<'a>,
}

impl<'a> Image<'a> {
    /// Creates a new image object from a byte array. 
    pub fn new(data: &'a [u8]) -> MorphResult<Self> {
        let data = Bmp::from_slice(data).map_err(|_| MorphError::Create("Image::data: Could not create image."))?;

        Ok(Image {
            data
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