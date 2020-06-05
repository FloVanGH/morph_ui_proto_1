use core::marker::PhantomData;

use heapless::{consts::*, String, Vec};

pub use self::platform::log;

use crate::{
    core::{Context, View, WidgetId},
    embedded_graphics::{
        fonts::{Font8x16, Text},
        image::{Image, ImageRaw, ImageRawLE},
        pixelcolor::PixelColor,
        prelude::*,
        primitives::*,
        style::*,
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
pub struct Shell<Message: 'static, D: DrawTarget<C> + 'static, C: 'static, V: 'static>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    V: View<Message>,
{
    is_running: bool,
    render: bool,
    draw_target: D,
    context: Context<Message>,
    view: Option<V>,
    _phantom: PhantomData<C>,
}

impl<Message, D: DrawTarget<C>, C, V> Shell<Message, D, C, V>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    V: View<Message>,
{
    /// Creates a new shell with a given render target.
    pub fn new(draw_target: D) -> Self {
        Shell {
            is_running: true,
            render: true,
            draw_target,
            context: Context::new(),
            view: None,
            _phantom: PhantomData::default(),
        }
    }

    fn init(&mut self) -> MorphResult<()> {
        let mut ctx = Context::new();
        if let Some(view) = &mut self.view {
            let root = view.view(&mut ctx)?;
            ctx.push(None, root)?;
        }

        self.context = ctx;

        Ok(())
    }

    pub fn view(mut self, view: V) -> Self {
        self.view = Some(view);
        self
    }

    // Drain events.
    fn drain_events(&mut self) -> MorphResult<()> {
        Ok(())
    }

    // Updates everything.
    fn update(&mut self) -> MorphResult<()> {
        Ok(())
    }

    pub fn int_draw(&mut self, id: WidgetId) -> MorphResult<()> {
        if let Some(widget) = self.context.get(id) {
            log(widget.name.as_str());

            for i in 0..widget.drawables.len() {
                match widget.drawables.get(i).unwrap().clone() {
                    crate::core::Drawable::Rectangle => {
                        let rectangle = Rectangle::new(Point::new(0, 0), Point::new(100, 100));

                        rectangle
                            .into_styled(
                                PrimitiveStyleBuilder::new()
                                    .fill_color(C::from(C::Raw::from_u32(
                                        Color::from("#000000").data,
                                    )))
                                    .build(),
                            )
                            .draw(&mut self.draw_target)
                            .map_err(|_| MorphError::Backend("Could not draw rectangle."))?;
                    }
                    crate::core::Drawable::Line => {
                        let line = Line::default();

                        line.into_styled(PrimitiveStyleBuilder::new().build())
                            .draw(&mut self.draw_target)
                            .map_err(|_| MorphError::Backend("Could not draw line."))?;
                    }
                    crate::core::Drawable::Circle => {
                        let circle = Circle::default();

                        circle
                            .into_styled(PrimitiveStyleBuilder::new().build())
                            .draw(&mut self.draw_target)
                            .map_err(|_| MorphError::Backend("Could not draw circle."))?;
                    }
                    crate::core::Drawable::Triangle => {
                        let triangle = Triangle::default();

                        triangle
                            .into_styled(PrimitiveStyleBuilder::new().build())
                            .draw(&mut self.draw_target)
                            .map_err(|_| MorphError::Backend("Could not draw triangle."))?;
                    }
                    crate::core::Drawable::Text => {
                        let text = if let Some(text) = &widget.text {
                            text.clone()
                        } else {
                            String::default()
                        };

                        let text = Text::new(text.as_str(), Point::default());

                        text.into_styled(
                            TextStyleBuilder::new(Font8x16)
                                .text_color(C::from(C::Raw::from_u32(Color::from("#f20b8e").data)))
                                .build(),
                        )
                        .draw(&mut self.draw_target)
                        .map_err(|_| MorphError::Backend("Could not draw text."))?;
                    }
                    crate::core::Drawable::Image => {
                        if widget.image.is_none() {
                            return Ok(());
                        }

                        log("image");

                        let image_raw: ImageRawLE<C> = ImageRaw::new(
                            widget.image.unwrap(),
                            widget.size.width,
                            widget.size.height,
                        );
                        let image: Image<_, C> = Image::new(&image_raw, Point::new(34, 8));
                        image
                            .draw(&mut self.draw_target)
                            .map_err(|_| MorphError::Backend(""))?;
                    }
                }
            }
        };

        if let Some(children_len) = self.context.children_len(id) {
            for i in 0..children_len {
                if let Some(child) = self.context.get_child_id(id, i).map(|i| *i) {
                    self.int_draw(child)?;
                }
            }
        }

        Ok(())
    }

    // Draws everything.
    fn draw(&mut self) -> MorphResult<()> {
        if self.render {
            if let Some(root) = self.context.root() {
                self.int_draw(root)?;
            }

            // let color = Color::from("#000000");
            // let style = PrimitiveStyleBuilder::new()
            //     .fill_color(C::from(C::Raw::from_u32(color.data)))
            //     .build();
            // let black_backdrop =
            //     Rectangle::new(Point::new(0, 0), Point::new(160, 128)).into_styled(style);
            // black_backdrop
            //     .draw(&mut self.draw_target)
            //     .map_err(|_| MorphError::Backend(""))?;

            // let color = Color::from("#ffffff");
            // // Create a new text style
            // let style = TextStyleBuilder::new(Font8x16)
            //     .text_color(C::from(C::Raw::from_u32(color.data)))
            //     .build();

            // // Create a text at position (20, 30) and draw it using the previously defined style
            // Text::new("Hello Rust!", Point::new(20, 100))
            //     .into_styled(style)
            //     .draw(&mut self.draw_target)
            //     .map_err(|_| MorphError::Backend(""))?;

            // let image_raw: ImageRawLE<C> =
            //     ImageRaw::new(include_bytes!("../../assets/ferris.raw"), 86, 64);
            // let image: Image<_, C> = Image::new(&image_raw, Point::new(34, 8));
            // image
            //     .draw(&mut self.draw_target)
            //     .map_err(|_| MorphError::Backend(""))?;
            self.render = false;
        }

        Ok(())
    }

    /// Start and run the shell.
    pub fn start(mut self) -> MorphResult<()> {
        platform::main_loop(move |running| {
            if !self.is_running {
                *running = false;
                return Ok(());
            }
            self.drain_events()?;
            self.update()?;
            self.init()?;
            self.draw()?;

            Ok(())
        })
    }
}
