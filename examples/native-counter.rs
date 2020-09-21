use heapless::{consts::*, String};

use morph::prelude::*;
use morph_native::NativeShell;

use morph::theme::Theme;

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
                Image::new(include_bytes!("../assets/ferris.raw"), 86, 64)?.margin(4),
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

fn main() -> MorphResult<()> {
    NativeShell::new(640, 240)?.view(Counter::default())?.run()
}
