use heapless::{consts::*, String};

use crate::{core::Widget, result::*};

#[derive(Debug, Clone)]
pub struct Text {
    text: String<U16>
}

impl Default for Text {
    fn default() -> Self {
        Text {
            text: String::default()
        }
    }
}

impl Text {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: impl Into<String<U16>>) -> Self {
        self.text = text.into();
        self
    }
}

impl<Message> IntoResult<Widget<Message>> for Text {
    fn into(self) -> MorphResult<Widget<Message>> {
        let mut widget = Widget::new()?;
        widget.text = Some(self.text);
        Ok(widget)
    }  
}