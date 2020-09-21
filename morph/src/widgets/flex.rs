use stretch::{style::{Style as LayoutStyle, Display, FlexDirection, Dimension}, geometry::{Size}};

use crate::{
    core::{get_widget_id, BaseStyle, Context, Widget, WidgetId},
    geometry::Thickness,
    result::*,
};

#[derive(Debug, Default)]
pub struct Flex {
    id: WidgetId,
    layout_style: LayoutStyle,
}

impl Flex {
    pub fn new() -> MorphResult<Self> {
        let layout_style = LayoutStyle {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            ..Default::default()
        };

        Ok(Flex {
            id: get_widget_id()?,
            layout_style,
        })
    }
}

impl Flex {
    pub fn child<Message, S>(
        self,
        context: &mut Context<Message, S>,
        child: impl IntoResult<Widget<Message, S>>,
    ) -> MorphResult<Self> where S: BaseStyle
    { 
        context.push(Some(self.id), child.into_result()?)?;
        Ok(self)
    }

    pub fn margin(mut self, margin: impl Into<Thickness>) -> Self {
        self.layout_style.margin = margin.into().into();
        self
    }
}

impl<Message, S> IntoResult<Widget<Message, S>> for Flex where S: BaseStyle
{
    fn into_result(self) -> MorphResult<Widget<Message, S>> {
        let mut widget = Widget::from_id(self.id)?;
        widget.name = "Flex";
        widget.layout_style = self.layout_style;
        Ok(widget)
    }
}
