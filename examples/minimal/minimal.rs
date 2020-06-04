use morph::prelude::*;

pub enum Message {
    Tapped,
}

pub fn start_example<D: DrawTarget<C> + 'static, C: 'static>(draw_target: D) -> MorphResult<()>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    Shell::new(draw_target)
        .view(
            Flex::new()
                .child(Button::new().on_tap(Message::Tapped).text("Tap me"))?
                .child(Label::new().text("Hello from morph."))?,
        )
        .start()
}
