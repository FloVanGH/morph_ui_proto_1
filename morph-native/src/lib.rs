use raqote_display::prelude::*;

use embedded_graphics::pixelcolor::Rgb565;

use orbclient::*;

use morph::prelude::*;

use std::time::Instant;

pub struct NativeShell<Message: 'static, V: 'static, S: 'static>
where
    V: View<Message, S>,
    S: IntoStyle,
{
    shell: Shell<Message, RaqoteDisplay, Rgb565, V, S>,
    window: Window,
}

impl<Message: 'static, V: 'static, S: 'static> NativeShell<Message, V, S>
where
    V: View<Message, S>,
    S: IntoStyle,
{
    pub fn new(width: u32, height: u32) -> MorphResult<NativeShell<Message, V, S>> {
        let shell = Shell::new().display(
            RaqoteDisplay::new(width, height)
                .map_err(|_| MorphError::Create("Cannot create raqote display."))?,
        );

        let (screen_width, screen_height) = orbclient::get_display_size().unwrap();

        let window_flags = vec![WindowFlag::Async];
        let window = Window::new_flags(
            ((screen_width - width) / 2) as i32,
            ((screen_height - height) / 2) as i32,
            width,
            height,
            "morph native shell",
            &window_flags,
        )
        .ok_or(0)
        .map_err(|_| MorphError::Create("Cannot create orbclient window."))?;

        Ok(NativeShell { shell, window })
    }

    pub fn view(mut self, view: V) -> MorphResult<Self> {
        self.shell = self.shell.view(view);
        Ok(self)
    }

    pub fn run(&mut self) {
        // let mut loop_started = Instant::now();
        'events: loop {
            self.shell.run();

            let color_data: Vec<orbclient::Color> = self
                .shell
                .display_ref()
                .unwrap()
                .data()
                .iter()
                .map(|p| orbclient::Color { data: *p })
                .collect();

            self.window
                .data_mut()
                .clone_from_slice(color_data.as_slice());
            self.window.sync();

            for event in self.window.events() {
                match event.to_option() {
                    EventOption::Quit(_quit_event) => break 'events,
                    // EventOption::Button(evt) => {
                    //     if !evt.left {
                    //         ui.event_send(&mut button, Event::Clicked).unwrap();
                    //     }
                    // }
                    _ => {}
                }
            }

            // self.shell.tick(loop_started.elapsed());
            // loop_started = Instant::now();
        }
    }
}
