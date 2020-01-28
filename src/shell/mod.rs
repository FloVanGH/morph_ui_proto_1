pub use self::platform::log;

use crate::platform;

/// The `Shell` is the main entry point of your application. It could compare with a combination of an application and window struct.
/// The Shell runs always in full screen and could be draw a background. It also runs the application, handles events, execute updates
/// and drawing. It is possible to operate the shell with different backend for different embedded devices. morph provides a default
/// set of backend e.g. for WebAssembly and cortex-m processors.
pub struct Shell {
    is_running: bool
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            is_running: true
        }
    }
}

impl Shell {
    // Drain events.
    fn drain_events(&mut self) {

    }

    // Updates everything.
    fn update(&mut self) {
       
    }

    // Draws everything.
    fn draw(&mut self) {

    }

    /// Start and run the application.
    pub fn start(mut self) {
        log("Start");
        platform::main_loop(move || {    
            if !self.is_running {
                return;
            }
            self.drain_events();
            self.update();
            self.draw();
        });
    }
}