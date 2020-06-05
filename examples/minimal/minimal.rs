use morph::prelude::*;

pub enum Message {
    Tapped,
}

pub fn start_example<D: DrawTarget<C> + 'static, C: 'static>(draw_target: D) -> MorphResult<()>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    Shell::new(draw_target)
        .view(|ctx| {
            Flex::new()?
                .margin(4)
                .child(
                    ctx,
                    Button::new()
                        .text("Tap me")?
                        .margin((0, 0, 8, 0))
                        .on_tap(Message::Tapped),
                )?
                .child(ctx, Label::new().text("Hello from morph.")?)?
                .into_result()
        })?
        .start()
}
