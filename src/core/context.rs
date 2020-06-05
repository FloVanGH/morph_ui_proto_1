use heapless::{consts::*, FnvIndexMap, Vec};

use crate::result::*;

use super::{Widget, WidgetId};

pub static MAXIMUM_WIDGETS: &'static str =
    "Maximum of widgets is reached. Could not add more widgets to context.";

#[derive(Default)]
pub struct Context<Message: 'static> {
    widgets: FnvIndexMap<WidgetId, Widget<Message>, U16>,
    parent: FnvIndexMap<WidgetId, WidgetId, U16>,
    root: Option<WidgetId>,
    children: FnvIndexMap<WidgetId, Vec<WidgetId, U8>, U16>,
}

impl<Message> Context<Message> {
    pub fn new() -> Self {
        Context {
            widgets: FnvIndexMap::new(),
            parent: FnvIndexMap::new(),
            children: FnvIndexMap::new(),
            root: None,
        }
    }

    pub fn push(&mut self, parent: Option<WidgetId>, widget: Widget<Message>) -> MorphResult<()> {
        let id = widget.id();

        if parent.is_none() {
            self.root = Some(id);
        }

        if !self.children.contains_key(&id) {
            self.children
                .insert(id, Vec::new())
                .map(|_| ())
                .map_err(|_| MorphError::OutOfBounds(MAXIMUM_WIDGETS))?;
        }

        if let Some(parent) = parent {
            self.parent
                .insert(id, parent)
                .map(|_| ())
                .map_err(|_| MorphError::OutOfBounds(MAXIMUM_WIDGETS))?;
        }

        self.widgets
            .insert(id, widget)
            .map(|_| ())
            .map_err(|_| MorphError::OutOfBounds(MAXIMUM_WIDGETS))
    }

    pub fn root(&self) -> Option<WidgetId> {
        self.root
    }

    pub fn children_len(&self, parent: WidgetId) -> Option<usize> {
        if let Some(children) = self.children.get(&parent) {
            return Some(children.len())
        }

        None
    }

    pub fn get_child_id(&self, parent: WidgetId, index: usize) -> Option<&WidgetId> {
        if let Some(children) = self.children.get(&parent) {
            return children.get(index)
        }

        None
    }

    pub fn get(&self, id: WidgetId) -> Option<&Widget<Message>> {
        self.widgets.get(&id)
    }

    pub fn get_mut(&mut self, id: WidgetId) -> Option<&mut Widget<Message>> {
        self.widgets.get_mut(&id)
    }

    pub fn remove(&mut self, id: WidgetId) -> Option<Widget<Message>> {
        let _ = self.children.remove(&id);
        let _ = self.parent.remove(&id);
        self.widgets.remove(&id)
    }

    pub fn len(&self) -> usize {
        self.widgets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.widgets.is_empty()
    }
}