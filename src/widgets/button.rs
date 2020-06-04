use heapless::{consts::*, String};

use crate::{core::Widget, result::*};

#[derive(Debug, Clone)]
pub struct Button<Message> {
    on_tap: Option<Message>,
    text: String<U16>,
}

impl<Message> Default for Button<Message> {
    fn default() -> Self {
        Button {
            on_tap: None,
            text: String::default(),
        }
    }
}

impl<Message> Button<Message> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: impl Into<String<U16>>) -> Self {
        self.text = text.into();
        self
    }

    pub fn on_tap(mut self, message: Message) -> Self {
        self.on_tap = Some(message);
        self
    }
}

impl<Message> IntoResult<Widget<Message>> for Button<Message> {
    fn into(self) -> MorphResult<Widget<Message>> {
        let mut widget = Widget::new()?;
        widget.text = Some(self.text);
        Ok(widget)
    }
}
