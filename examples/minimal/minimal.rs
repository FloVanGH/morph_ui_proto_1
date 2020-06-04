use morph::prelude::*;

pub enum Message {
    Tapped,
}

pub fn start_example<D: DrawTarget<C> + 'static, C: 'static>(draw_target: D)
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    let flex = Flex::new().child(Button::new().on_tap(Message::Tapped).text("Tap me"));
    let _result = Shell::new(draw_target).start();
}
