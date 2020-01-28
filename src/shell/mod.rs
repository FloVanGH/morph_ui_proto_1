pub use self::platform::log;

use crate::platform;
use crate::result::*;

/// The `Shell` is the main entry point of your application. It could compare with a combination of an application and window struct.
/// The Shell runs always in full screen and could be draw a background. It also runs the application, handles events, execute updates
/// and drawing. It is possible to operate the shell with different backend for different embedded devices. morph provides a default
/// set of backend e.g. for WebAssembly and cortex-m processors.
pub struct Shell {
    is_running: bool,
}

impl Shell {
    pub fn new() -> Self {
        Shell { is_running: true }
    }
}

impl Shell {
    // Drain events.
    fn drain_events(&mut self) -> MorphResult<()> {
        Ok(())
    }

    // Updates everything.
    fn update(&mut self) -> MorphResult<()> {
        Ok(())
    }

    // Draws everything.
    fn draw(&mut self) -> MorphResult<()> {
        Ok(())
    }

    /// Start and run the application.
    pub fn start(mut self)  -> MorphResult<()> {
        log("Start");
        platform::main_loop(move || {
            if !self.is_running {
                return Ok(());
            }
            self.drain_events()?;
            self.update()?;
            self.draw()?;

            Ok(())
        })
    }
}
