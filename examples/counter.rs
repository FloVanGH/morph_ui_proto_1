use morph::{embedded_graphics::{mock_display::MockDisplay}, platform::RaqoteBackend, prelude::*};
pub use morph::theme::Theme;

#[path = "counter/counter.rs"]
mod counter;

pub fn image() -> MorphResult<morph::widgets::Image> {
    Image::new(include_bytes!("../assets/ferris.raw"), 64, 64)
}

fn main() {
    let mut shell =  counter::shell(160, 120, RaqoteBackend::new(MockDisplay::new()));
    loop {
        let result = shell.run();

        if let Ok(running) = result {
            if !running {
                break;
            }
        }

        if let Err(err) = result {
            panic!("{:?}", err);
        }
    }   
}