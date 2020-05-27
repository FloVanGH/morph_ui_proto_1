pub use self::platform::log;

use core::marker::PhantomData;

use tinybmp::Bmp;


use crate::{
    geometry::Size,
    embedded_graphics::{DrawTarget, pixelcolor::PixelColor},
    graphics::{Image, RenderContext, RenderTarget},
    platform,
    result::*,
};

// /// Creates platform specific shell with a platform specific render target.
// pub fn shell() -> MorphResult<Shell<DrawTarget, platform::RenderTarget, platform::RenderContext>> {
//     Ok(Shell::new(platform::RenderTarget::new()?))
// }

/// The `Shell` is the main entry point of your application. It could compare with a combination of an application and window struct.
/// The Shell runs always in full screen and could be draw a background. It also runs the application, handles events, execute updates
/// and drawing. It is possible to operate the shell with different backend for different embedded devices. morph provides a default
/// set of backend e.g. for WebAssembly and cortex-m processors.
pub struct Shell<D: 'static, C: 'static>
where
    D: DrawTarget<C>,
    C: PixelColor
{
    is_running: bool,
    render: bool,
    draw_target: D,
    _phantom: PhantomData<C>,
}

impl<D: 'static, C: 'static> Shell<D, C>
where
    D: DrawTarget<C>,
    C: PixelColor
{
    /// Creates a new shell with a given render target.
    pub fn new(draw_target: D) -> Self {
        Shell {
            is_running: true, 
            render: true,
            draw_target,
            _phantom: PhantomData::default(),
        }
    }

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
        if self.render {
         
            self.render = false;
        }
        Ok(())
    }

    /// Sets the size the application shell should use to draw on the screen.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        // self.render_target.set_size(size);
        self
    }

    /// Start and run the shell.
    pub fn start(mut self) -> MorphResult<()> {
        log("Start");

        platform::main_loop(move |running| {
            if !self.is_running {
                *running = false;
                return Ok(());
            }
            self.drain_events()?;
            self.update()?;
            self.draw()?;

            Ok(())
        })
    }
}