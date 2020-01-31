#![no_std]
#![no_main]

#[path = "minimal.rs"]
mod minimal;

extern crate panic_halt;

use embedded_graphics::fonts::{Font6x8, Text};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};
use embedded_graphics::style::{PrimitiveStyle, TextStyle};

// Only used for examples - this would be replaced by the driver for your chosen display
use embedded_graphics::mock_display::MockDisplay as Display;
use cortex_m_rt::entry;
use cortex_m_semihosting::debug;

#[entry]
fn main() -> ! {
     // Create a display object to draw into
    // This will be whichever display driver you decide to use, like the SSD1306, SSD1351, etc
    let mut display = Display::new();

    // Draw a circle centered at (64, 64) with a radius of 64 and a white 1px stroke
    Circle::new(Point::new(64, 64), 64)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
        .draw(&mut display);

    // Draw a white 1px thick line from (64, 64) to (0, 64)
    Line::new(Point::new(64, 64), Point::new(0, 64))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
        .draw(&mut display);

    // Draw a red 1px line from (64, 64) to (80, 80)
    Line::new(Point::new(64, 64), Point::new(80, 80))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
        .draw(&mut display);

    // Print "Hello world!" in a white 6x8 pixel font with the top left corner positioned at (5, 50)
    Text::new("Hello World!", Point::new(5, 50))
        .into_styled(TextStyle::new(Font6x8, Rgb565::WHITE))
        .draw(&mut display);
        
    minimal::start_example();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}