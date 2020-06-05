use embedded_graphics::{mock_display::MockDisplay, pixelcolor::BinaryColor};

#[path = "counter/counter.rs"]
mod counter;

fn main() {
    counter::start_example::<MockDisplay<BinaryColor>, BinaryColor>(MockDisplay::new());
}