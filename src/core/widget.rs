use stretch::style::Style;
use heapless::{consts::*, String, Vec};

use crate::result::*;

pub type WidgetId = u8;

// Maximal number of widgets that are allowed to create.
const MAX_WIDGETS: usize = 10;

// Used to generate unique widget id.
static mut WIDGET_ID: WidgetId = 0;

// Returns an unique widget id.
fn get_widget_id() -> MorphResult<WidgetId> {
    let id = unsafe { WIDGET_ID };

    if id as usize > MAX_WIDGETS {
        return Err(MorphError::OutOfBounds("Maximum of widgets is reached. Could not create more widgets."));
    }

    unsafe { WIDGET_ID += 1; }

    Ok(id)
}

#[derive(Debug, Clone)]
pub struct Widget<Message> {
    id: WidgetId,
    pub is_dirty: bool,
    pub text: Option<String<U16>>,
    pub on_tap: Option<Message>,
    pub layout_style: Style,
}

impl<Message> Widget<Message> {
    pub fn new() -> MorphResult<Self> {
        let id = get_widget_id()?;

        Ok(Widget {
            id,
            is_dirty: true,
            text: None,
            on_tap: None,
            layout_style: Style::default()
        })
    }
}