use crate::result::*;

use super::{Widget, Context};

pub trait View<Message> {
    fn update(&mut self, message: Message);
    fn view(&self, ctx: &mut Context<Message>) -> MorphResult<Widget<Message>>;
}