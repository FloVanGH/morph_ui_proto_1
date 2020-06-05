use morph::prelude::*;

enum Message {
    Increment,
}

#[derive(Default, Debug)]
pub struct Counter {
    count: u32,
}

impl View<Message, Theme> for Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.count += 1;
            }
        }
    }

    fn view(&self, ctx: &mut Context<Message, Theme>) -> MorphResult<Widget<Message, Theme>> {
        Flex::new()?
            .margin(4)
            .child(
                ctx,
                Button::new()
                    .text("Increment")?
                    .margin((0, 0, 8, 0))
                    .on_tap(Message::Increment),
            )?
            .child(
                ctx,
                Image::new(include_bytes!("../../assets/ferris.raw"), 86, 64)?.margin(4),
            )?
            .child(ctx, Label::new().text(self.count)?)?
          
            .into_result()
    }
}

pub fn start_example<D: DrawTarget<C> + 'static, C: 'static>(draw_target: D) -> MorphResult<()>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    Shell::new(draw_target).view(Counter::default()).start()
}
