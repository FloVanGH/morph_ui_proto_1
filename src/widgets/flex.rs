use crate::{core::Widget, result::*};
use heapless::{consts::*, Vec};

#[derive(Debug, Clone)]
pub struct Flex<Message> {
    children: Vec<Widget<Message>, U8>,
}

impl<Message> Flex<Message> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Message> Default for Flex<Message> {
    fn default() -> Self {
        Flex {
            children: Vec::default()
        }
    }
}

impl<Message> Flex<Message>{
    pub fn child(mut self, child: impl IntoResult<Widget<Message>>) -> MorphResult<Self> {
        self.children
            .push(child.into()?)
            .map_err(|_| MorphError::OutOfBounds("Flex::child: could not add more children."))?;
        Ok(self)
    }
}

impl<Message> IntoResult<Widget<Message>> for Flex<Message> {
    fn into(self) -> MorphResult<Widget<Message>> {
        Widget::new()
    }
}
