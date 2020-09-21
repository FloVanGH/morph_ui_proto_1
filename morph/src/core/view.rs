use crate::result::*;

use super::{Widget, Context, IntoStyle};

pub trait View<Message, S> {
    fn update(&mut self, message: Message);
    fn view(&self, ctx: &mut Context<Message, S>) -> MorphResult<Widget<Message, S>> where S: IntoStyle;
}