use morph::prelude::*;

pub fn start_example<D: DrawTarget<C> + 'static, C: 'static>(draw_target: D)
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    let _result = Shell::new(draw_target).start();
}
