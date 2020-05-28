pub use self::platform::log;

use core::marker::PhantomData;

use tinybmp::Bmp;

use crate::{
    embedded_graphics::{
        image::{Image, ImageRaw, ImageRawLE},
        pixelcolor::PixelColor,
        pixelcolor::Rgb565,
        prelude::*,
        primitives::rectangle::Rectangle,
        style::PrimitiveStyleBuilder,
        DrawTarget,
    },
    graphics::Color,
    geometry::Size,
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
pub struct Shell<D: DrawTarget<C> + 'static, C: 'static>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    is_running: bool,
    render: bool,
    draw_target: D,
    _phantom: PhantomData<C>,
}

impl<D: DrawTarget<C> + 'static, C: 'static> Shell<D, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
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
            // let color = Color::from("#005eff");
            // let style = PrimitiveStyleBuilder::new()
            //     .fill_color(C::from(C::Raw::from_u32(color.data)))
            //     .build();
            // let black_backdrop =
            //     Rectangle::new(Point::new(0, 0), Point::new(160, 128)).into_styled(style);
            // black_backdrop.draw(&mut self.draw_target).map_err(|_| MorphError::Backend(""))?;
    
            let image_raw: ImageRawLE<C> =
                ImageRaw::new(include_bytes!("../../assets/ferris.raw"), 86, 64);
            let image: Image<_, C> = Image::new(&image_raw, Point::new(34, 8));
            image.draw(&mut self.draw_target).map_err(|_| MorphError::Backend(""))?;
            self.render = false;
        }
        
        Ok(())
    }

    /// Sets the size the application shell should use to draw on the screen.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
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
