use core::marker::PhantomData;

pub use self::platform::log;

use crate::{
    core::{Widget, Context},
    embedded_graphics::{
        fonts::{Font8x16, Text},
        image::{Image, ImageRaw, ImageRawLE},
        pixelcolor::PixelColor,
        prelude::*,
        primitives::Rectangle,
        style::{PrimitiveStyleBuilder, TextStyle, TextStyleBuilder},
        DrawTarget,
    },
    graphics::Color,
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
pub struct Shell<Message: 'static, D: DrawTarget<C> + 'static, C: 'static>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    is_running: bool,
    render: bool,
    draw_target: D,
    context: Context<Message>,
    _phantom: PhantomData<C>
}

impl<Message, D: DrawTarget<C> + 'static, C: 'static> Shell<Message, D, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    /// Creates a new shell with a given render target.
    pub fn new(draw_target: D) -> Self {
        Shell {
            is_running: true,
            render: true,
            draw_target,
            context: Context::new(),
            _phantom: PhantomData::default()
        }
    }

    pub fn view<F: Fn(&mut Context<Message>) -> MorphResult<Widget<Message>> + 'static>(mut self, build_fn: F) -> MorphResult<Self> {
        let root = build_fn(&mut self.context)?;
        self.context.push(None, root)?;
        Ok(self)
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

            if let Some(root) = self.context.root() {
                
            }

            let color = Color::from("#000000");
            let style = PrimitiveStyleBuilder::new()
                .fill_color(C::from(C::Raw::from_u32(color.data)))
                .build();
            let black_backdrop =
                Rectangle::new(Point::new(0, 0), Point::new(160, 128)).into_styled(style);
            black_backdrop
                .draw(&mut self.draw_target)
                .map_err(|_| MorphError::Backend(""))?;

            let color = Color::from("#ffffff");
            // Create a new text style
            let style = TextStyleBuilder::new(Font8x16)
                .text_color(C::from(C::Raw::from_u32(color.data)))
                .build();

            // Create a text at position (20, 30) and draw it using the previously defined style
            Text::new("Hello Rust!", Point::new(20, 100))
                .into_styled(style)
                .draw(&mut self.draw_target)
                .map_err(|_| MorphError::Backend(""))?;

            let image_raw: ImageRawLE<C> =
                ImageRaw::new(include_bytes!("../../assets/ferris.raw"), 86, 64);
            let image: Image<_, C> = Image::new(&image_raw, Point::new(34, 8));
            image
                .draw(&mut self.draw_target)
                .map_err(|_| MorphError::Backend(""))?;
            self.render = false;
        }

        Ok(())
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
