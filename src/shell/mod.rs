pub use self::platform::log;

use core::marker::PhantomData;

use crate::{
    geometry::Size,
    graphics::{RenderContext, RenderTarget},
    platform,
    result::*,
};

/// Creates platform specific shell with a platform specific render target.
pub fn shell() -> MorphResult<Shell<platform::RenderTarget, platform::RenderContext>> {
    Ok(Shell::new(platform::RenderTarget::new()?))
}

/// The `Shell` is the main entry point of your application. It could compare with a combination of an application and window struct.
/// The Shell runs always in full screen and could be draw a background. It also runs the application, handles events, execute updates
/// and drawing. It is possible to operate the shell with different backend for different embedded devices. morph provides a default
/// set of backend e.g. for WebAssembly and cortex-m processors.
pub struct Shell<R: 'static, C: 'static>
where
    R: RenderTarget<C>,
    C: RenderContext,
{
    is_running: bool,
    render: bool,
    render_target: R,
    _phantom: PhantomData<C>
}

impl<R, C> Shell<R, C>
where
    R: RenderTarget<C>,
    C: RenderContext,
{
    /// Creates a new shell with a given render target.
    pub fn new(render_target: R) -> Self {
        Shell {
            is_running: true,
            render_target,
            render: true,
            _phantom: PhantomData::default()
        }
    }
}

impl<R, C> Shell<R, C>
where
    R: RenderTarget<C>,
    C: RenderContext,
{
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
            let mut render_context = self.render_target.context()?;
            render_context.begin_path();
            render_context.set_fill_style("#8d03e2");
            render_context.fill_rect((0, 0), (50, 50));
            render_context.move_to((10, 10));
            render_context.line_to((100, 100));
            render_context.set_stroke_style("##0040ff");
            render_context.stroke();
            render_context.set_fill_style("#ff0000");
            render_context.fill_circle((30, 30), 20);
            render_context.set_stroke_style("##00ff00");
            render_context.stroke_circle((15, 15), 10);
            render_context.set_stroke_style("#0000ff");
            render_context.stroke_triangle((10, 10), (20, 20), (0, 20));
            render_context.set_fill_style("#40ff00");
            render_context.fill_triangle((10, 30), (60, 60), (10, 60));
            self.render_target.draw_to_screen(render_context);
            self.render = false;
        }
        Ok(())
    }

    /// Sets the size the application shell should use to draw on the screen.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.render_target.set_size(size);
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
