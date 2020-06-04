use crate::{core::Widget, result::*};
use heapless::{consts::*, Vec};

#[derive(Debug, Default, Clone)]
pub struct Flex {
    children: Vec<Widget, U8>,
}

impl Flex {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Flex {
    pub fn child(mut self, child: impl IntoResult<Widget>) -> MorphResult<Self> {
        self.children
            .push(child.into()?)
            .map_err(|_| MorphError::OutOfBounds("Flex::child: could not add more children."))?;
        Ok(self)
    }
}

impl IntoResult<Widget> for Flex {
    fn into(self) -> MorphResult<Widget> {
        Widget::new()
    }
}
