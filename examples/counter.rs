use embedded_graphics::{pixelcolor::BinaryColor};
use morph::{embedded_graphics::{pixelcolor::Rgb565, mock_display::MockDisplay}, platform::RaqoteBackend};
pub use morph::theme::Theme;

#[path = "counter/counter.rs"]
mod counter;

fn main() {
    let mut shell =  counter::shell(RaqoteBackend::new(MockDisplay::new()));
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