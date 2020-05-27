use embedded_graphics::{mock_display::MockDisplay, pixelcolor::BinaryColor};

#[path = "minimal/minimal.rs"]
mod minimal;

fn main() {
    minimal::start_example::<MockDisplay<BinaryColor>, BinaryColor>(MockDisplay::new());
}