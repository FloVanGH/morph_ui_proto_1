use morph::prelude::*;

pub fn start_example<D: 'static, C: 'static>(draw_target: D) where D: DrawTarget<C>, C: PixelColor {
    let _result = Shell::new(draw_target).size((100, 100)).start();
}
