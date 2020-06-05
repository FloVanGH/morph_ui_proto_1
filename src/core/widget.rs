use embedded_graphics::pixelcolor::PixelColor;

use stretch::style::Style;
use heapless::{consts::*, String, Vec};

use crate::result::*;

use super::Drawable;

pub type WidgetId = u8;

// Maximal number of widgets that are allowed to create.
const MAX_WIDGETS: usize = 10;

// Used to generate unique widget id.
static mut WIDGET_ID: WidgetId = 0;

// Returns an unique widget id.
pub fn get_widget_id() -> MorphResult<WidgetId> {
    let id = unsafe { WIDGET_ID };

    if id as usize > MAX_WIDGETS {
        return Err(MorphError::OutOfBounds("Maximum of widgets is reached. Could not create more widgets."));
    }

    unsafe { WIDGET_ID += 1; }

    Ok(id)
}

pub struct Widget<Message>  {
    id: WidgetId,
    pub name: String<U8>,
    pub is_dirty: bool,
    pub text: Option<String<U64>>,
    pub on_tap: Option<Message>,
    pub layout_style: Style,
    pub drawables: Vec<Drawable, U4>,
}

impl<Message> Widget<Message> {
    pub fn new() -> MorphResult<Self> {
       Self::from_id(get_widget_id()?)
    }

    pub fn from_id(id: WidgetId) -> MorphResult<Self> {
        Ok(Widget {
            id,
            name: String::new(),
            is_dirty: true,
            text: None,
            on_tap: None,
            layout_style: Style::default(),
            drawables: Vec::new(),
        })
    }

    pub fn id(&self) -> WidgetId {
        self.id
    }
}