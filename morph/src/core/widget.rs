use heapless::{consts::*, String, Vec};
use stretch::{style::Style as LayoutStyle, node::Node};

use crate::{embedded_graphics::geometry::Size, result::*};

use super::{Drawable, IntoStyle, State, Style};

pub type WidgetId = u8;

// Maximal number of widgets that are allowed to create.
const MAX_WIDGETS: usize = 10;

// Used to generate unique widget id.
static mut WIDGET_ID: WidgetId = 0;

// Returns an unique widget id.
pub fn get_widget_id() -> MorphResult<WidgetId> {
    let id = unsafe { WIDGET_ID };

    if id as usize > MAX_WIDGETS {
        return Err(MorphError::OutOfBounds(
            "Maximum of widgets is reached. Could not create more widgets.",
        ));
    }

    unsafe {
        WIDGET_ID += 1;
    }

    Ok(id)
}

pub(crate) fn reset_widget_id() {
    unsafe {
        WIDGET_ID = 0;
    }
}

pub struct Widget<Message, S>
where
    S: IntoStyle,
{
    id: WidgetId,
    pub name: &'static str,
    pub is_dirty: bool,
    pub text: Option<&'static str>,
    pub image: Option<&'static [u8]>,
    pub on_tap: Option<Message>,
    pub layout_style: LayoutStyle,
    pub drawables: Vec<Drawable, U4>,
    pub size: Size,
    pub style: Option<S>,
    pub state: Option<State>,
    pub node: Option<Node>,
}

impl<Message, S> Widget<Message, S>
where
    S: IntoStyle,
{
    pub fn new() -> MorphResult<Self> {
        Self::from_id(get_widget_id()?)
    }

    pub fn from_id(id: WidgetId) -> MorphResult<Self> {
        Ok(Widget {
            id,
            name: "",
            is_dirty: true,
            text: None,
            image: None,
            on_tap: None,
            layout_style: LayoutStyle::default(),
            drawables: Vec::new(),
            size: Size::default(),
            style: None,
            state: None,
            node: None
        })
    }

    pub fn id(&self) -> WidgetId {
        self.id
    }

    pub fn style(&self) -> Option<Style> {
        if let Some(style) = &self.style {
            return Some(style.into_style(self.state));
        }

        None
    }

    pub fn copy_state(&mut self, other: &Widget<Message, S>) {
        if self.id != other.id || self.name != other.name || other.state.is_none() {
            return;
        }

        self.state = other.state.clone();
    }
}
