pub use self::platform::log;

use crate::{platform, render::RenderTarget, result::*, geometry::Size};

/// The `Shell` is the main entry point of your application. It could compare with a combination of an application and window struct.
/// The Shell runs always in full screen and could be draw a background. It also runs the application, handles events, execute updates
/// and drawing. It is possible to operate the shell with different backend for different embedded devices. morph provides a default
/// set of backend e.g. for WebAssembly and cortex-m processors.
pub struct Shell {
    is_running: bool,
    size: Size
}

impl Shell {
    pub fn new() -> Self {
        Shell { is_running: true, size: Size::default() }
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
    fn draw(&mut self, render_target: &mut RenderTarget) -> MorphResult<()> {
        Ok(())
    }

    /// Sets the size the application shell should use to draw on the screen.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Start and run the shell.
    pub fn start(mut self)  -> MorphResult<()> {
        log("Start");
        let mut render_target = RenderTarget::new(self.size)?;

        platform::main_loop(move |running| {
            if !self.is_running {
                *running = false;
                return Ok(());
            }
            self.drain_events()?;
            self.update()?;
            self.draw(&mut render_target)?;

            Ok(())
        })
    }
}
