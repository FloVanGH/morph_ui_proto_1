use crate::{geometry::*, result::*};

/// Represents a raw image object.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Image<'a> {
    data: &'a [u8],
    size: Size, 
}

impl<'a> Image<'a> {
    /// Creates a new image object from a raw pixel slice with given size. 
    pub fn new(data: &'a [u8], size: impl Into<Size>) -> MorphResult<Self> {
        let size = size.into();

        if !(size.width() * size.height()).eq(&(data.len() as u32)) {
            return Err(MorphError::Create("Image::new: data has not the correct size."))
        }

        Ok(Image {
            data,
            size
        })
    }

    /// Gets the width of the image.
    pub fn width(&self) -> u32 {
        self.size.width()
    }

    /// Gets the height of the image.
    pub fn height(&self) -> u32 {
        self.size.height()
    }

    /// Gets the size (width / height) of the image.
    pub fn size(&self) -> Size {    
        self.size
    }

    /// Gets the number of pixels.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the image data is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Gets an pixel as u8 of the given position.
    pub fn get(&self, position: impl Into<Point>) -> MorphResult<u8> {
        let position = position.into();
        let index = position.x() * self.size.width() as i32 + position.x();

        if index < 0 || index as usize > self.len() {
            return Err(MorphError::OutOfBounds("Image::get: given position is out of bounds."));
        }

        Ok(self.data[index as usize])
    }
}