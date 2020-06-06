use morph::prelude::*;

use super::Theme;

use heapless::{consts::*, String};

static mut L: String<U16> = String(heapless::i::String::new());

fn count() -> &'static str {
    unsafe { L.as_str() }
}

fn set_count(count: i32) {
    unsafe {
        L = String::from(count);
    }
}

pub enum Message {
    Increment,
    Decrement,
}

#[derive(Default, Debug)]
pub struct Counter {
    count: i32,
}

impl View<Message, Theme> for Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.count += 1;
            }
            Message::Decrement => {
                self.count -= 1;
            }
        }

        set_count(self.count);
    }

    fn view(&self, ctx: &mut Context<Message, Theme>) -> MorphResult<Widget<Message, Theme>> {
        set_count(self.count);
        Flex::new()?
            .margin(4)
            .child(
                ctx,
                Image::new(include_bytes!("../../assets/rust.raw"), 64, 64)?.margin(4),
            )?
            .child(
                ctx,
                Button::new()
                    .text("-")
                    .margin((0, 0, 8, 0))
                    .on_tap(Message::Decrement),
            )?
            .child(ctx, Label::new().text(count()))?
            .child(
                ctx,
                Button::new()
                    .text("+")
                    .margin((0, 8, 0, 0))
                    .on_tap(Message::Increment),
            )?
            .into_result()
    }
}

pub fn shell<B: Backend<D, C>, D: DrawTarget<C> + 'static, C: 'static>(
    backend: B,
) -> Shell<Message, B, D, C, Counter, Theme>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    Shell::new(backend).view(Counter::default())
}
