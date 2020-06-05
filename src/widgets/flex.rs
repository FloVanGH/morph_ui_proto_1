use stretch::style::Style;

use crate::{
    core::{Context, Widget, WidgetId, get_widget_id},
    geometry::Thickness,
    result::*,
};

#[derive(Debug, Default)]
pub struct Flex {
    id: WidgetId,
    layout_style: Style,
}

impl Flex {
    pub fn new() -> MorphResult<Self> {
        Ok(Flex {
            id: get_widget_id()?,
            layout_style: Style::default()
        })
    }
}

impl Flex {
    pub fn child<Message>(
        self,
        context: &mut Context<Message>,
        child: impl IntoResult<Widget<Message>>,
    ) -> MorphResult<Self> {
        context.push(Some(self.id), child.into_result()?)?;
        Ok(self)
    }

    pub fn margin(mut self, margin: impl Into<Thickness>) -> Self {
        self.layout_style.margin = margin.into().into();
        self
    }
}

impl<Message> IntoResult<Widget<Message>> for Flex {
    fn into_result(self) -> MorphResult<Widget<Message>> {
        let mut widget = Widget::from_id(self.id)?;
        widget.name.push_str("Flex").map_err(|_| MorphError::OutOfBounds("Could not set name for flex."))?;
        widget.layout_style = self.layout_style;
        // widget.children = self.children;
        Ok(widget)
    }
}
