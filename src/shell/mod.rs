use core::marker::PhantomData;

use stretch::{geometry::Size, node::Node, number::Number, style::Dimension, Stretch};

use heapless::{consts::*, String, Vec};

pub use self::platform::log;

use crate::{
    core::{reset_widget_id, Context, IntoStyle, View, WidgetId},
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
    theme::Theme,
};

pub use self::backend::*;

mod backend;

// /// Get shell with default theme
// pub fn shell<D: DrawTarget<C> + 'static, Message, C, V>(draw_target: D) -> Shell<Message, D, C, V, Theme>
// where
//     C: PixelColor + From<<C as PixelColor>::Raw>,
//     V: View<Message, Theme>,
// {
//     Shell::new(draw_target)
// }

// /// Creates platform specific shell with a platform specific render target.
// pub fn shell() -> MorphResult<Shell<DrawTarget, platform::RenderTarget, platform::RenderContext>> {
//     Ok(Shell::new(platform::RenderTarget::new()?))
// }

/// The `Shell` is the main entry point of your application. It could compare with a combination of an application and window struct.
/// The Shell runs always in full screen and could be draw a background. It also runs the application, handles events, execute updates
/// and drawing. It is possible to operate the shell with different backend for different embedded devices. morph provides a default
/// set of backend e.g. for WebAssembly and cortex-m processors.
pub struct Shell<
    Message: 'static,
    B: Backend<D, C>,
    D: DrawTarget<C> + 'static,
    C: 'static,
    V: 'static,
    S: 'static,
> where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    V: View<Message, S>,
    S: IntoStyle,
{
    is_running: bool,
    render: bool,
    update: bool,
    backend: B,
    context: Context<Message, S>,
    size: (i32, i32),
    view: Option<V>,
    _phantom: PhantomData<C>,
    _phantom_d: PhantomData<D>,
}

impl<Message, B: Backend<D, C>, D: DrawTarget<C>, C, V, S> Shell<Message, B, D, C, V, S>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    V: View<Message, S>,
    S: IntoStyle,
{
    /// Creates a new shell with a given render target.
    pub fn new(backend: B) -> Self {
        let mut backend = backend;
        backend.init();
        Shell {
            is_running: true,
            render: true,
            update: true,
            backend,
            context: Context::new(),
            size: (0, 0),
            view: None,
            _phantom: PhantomData::default(),
            _phantom_d: PhantomData::default(),
        }
    }

    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.size = (width, height);
        self
    }

    // Copy states from old tree to new.
    //
    // ! This works only if the structure of the widget tree doesn't change.!
    // ! If this will changed in the future this logic must also be changed!
    fn copy_states(&mut self, id: WidgetId, new_ctx: &mut Context<Message, S>) {
        if let Some(widget) = self.context.get_mut(id) {
            if let Some(new_widget) = &mut new_ctx.get_mut(id) {
                new_widget.copy_state(widget);
            }
        }

        if let Some(children_len) = self.context.children_len(id) {
            for i in 0..children_len {
                if let Some(child) = self.context.get_child_id(id, i).map(|i| *i) {
                    self.copy_states(child, new_ctx);
                }
            }
        }
    }

    fn build(&mut self) -> MorphResult<()> {
        reset_widget_id();

        let mut ctx = Context::new();
        if let Some(view) = &mut self.view {
            let root = view.view(&mut ctx)?;
            ctx.push(None, root)?;
        }

        // by init.
        if !self.context.is_empty() {
            if let Some(root) = self.context.root() {
                self.copy_states(root, &mut ctx);
            }
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

    fn int_layout(
        &mut self,
        id: WidgetId,
        stretch: &mut Stretch,
        nodes: &mut Vec<Node, U16>,
    ) -> MorphResult<()> {
        let mut children: Vec<Node, U16> = Vec::new();

        if let Some(children_len) = self.context.children_len(id) {
            for i in 0..children_len {
                if let Some(child) = self.context.get_child_id(id, i).map(|i| *i) {
                    self.int_layout(child, stretch, nodes)?;
                    if nodes.len() > 0 {
                        if let Some(child_node) = nodes.get(nodes.len() - 1) {
                            children
                                .push(child_node.clone())
                                .map_err(|_| MorphError::OutOfBounds("To many children."))?;
                        }
                    }
                }
            }
        }

        if let Some(widget) = self.context.get(id) {
            let node = stretch
                .new_node(widget.layout_style.clone(), &children)
                .map_err(|_| MorphError::OutOfBounds("Could not generate stretch node."))?;
            return nodes
                .push(node)
                .map_err(|_| MorphError::OutOfBounds("Could not add more nodes to layout."));
        }

        Err(MorphError::Other("Could not initialize layout."))
    }

    // Updates everything.
    fn update(&mut self) -> MorphResult<()> {
        if !self.update {
            return Ok(());
        }

        let mut stretch = Stretch::new();
        let mut nodes = Vec::new();

        if let Some(root) = self.context.root() {
            self.int_layout(root, &mut stretch, &mut nodes)?;

            if nodes.len() > 0 {
                if let Some(node) = nodes.get(nodes.len() - 1) {
                    stretch
                        .compute_layout(
                            *node,
                            Size {
                                width: Number::Defined(self.size.0 as f32),
                                height: Number::Defined(self.size.1 as f32),
                            },
                        )
                        .map_err(|_| MorphError::Backend("Could not compute layout"))?;
                }
            }

            for node in nodes {
                log(format!("{:?}", stretch.layout(node).unwrap()).as_str());
            }
        }

        self.update = false;

        Ok(())
    }

    pub fn int_draw(&mut self, id: WidgetId) -> MorphResult<()> {
        if let Some(widget) = self.context.get(id) {
            let style = widget.style();

            for i in 0..widget.drawables.len() {
                match widget.drawables.get(i).unwrap().clone() {
                    crate::core::Drawable::Rectangle => {
                        let rectangle = Rectangle::new(Point::new(0, 0), Point::new(50, 30));

                        let mut style_builder = PrimitiveStyleBuilder::new();

                        if let Some(style) = style {
                            if let Some(background) = style.background {
                                style_builder = style_builder
                                    .fill_color(C::from(C::Raw::from_u32(background.data)));
                            }

                            if let Some(border_color) = style.border_color {
                                style_builder = style_builder
                                    .stroke_color(C::from(C::Raw::from_u32(border_color.data)));
                            }

                            if let Some(border_width) = style.border_width {
                                style_builder = style_builder.stroke_width(border_width);
                            }
                        }

                        rectangle
                            .into_styled(style_builder.build())
                            .draw(self.backend.draw_target())
                            .map_err(|_| MorphError::Backend("Could not draw rectangle."))?;
                    }
                    crate::core::Drawable::Line => {
                        let line = Line::default();

                        line.into_styled(PrimitiveStyleBuilder::new().build())
                            .draw(self.backend.draw_target())
                            .map_err(|_| MorphError::Backend("Could not draw line."))?;
                    }
                    crate::core::Drawable::Circle => {
                        let circle = Circle::default();

                        circle
                            .into_styled(PrimitiveStyleBuilder::new().build())
                            .draw(self.backend.draw_target())
                            .map_err(|_| MorphError::Backend("Could not draw circle."))?;
                    }
                    crate::core::Drawable::Triangle => {
                        let triangle = Triangle::default();

                        triangle
                            .into_styled(PrimitiveStyleBuilder::new().build())
                            .draw(self.backend.draw_target())
                            .map_err(|_| MorphError::Backend("Could not draw triangle."))?;
                    }
                    crate::core::Drawable::Text => {
                        if let Some(text) = widget.text {
                            let text = Text::new(text, Point::default());

                            let mut style_builder = TextStyleBuilder::new(Font8x16);

                            if let Some(style) = style {
                                if let Some(color) = style.color {
                                    style_builder = style_builder
                                        .text_color(C::from(C::Raw::from_u32(color.data)));
                                }
                            }

                            text.into_styled(style_builder.build())
                                .draw(self.backend.draw_target())
                                .map_err(|_| MorphError::Backend("Could not draw text."))?;
                        }
                    }
                    crate::core::Drawable::Image => {
                        if widget.image.is_none() {
                            return Ok(());
                        }

                        let image_raw: ImageRawLE<C> = ImageRaw::new(
                            widget.image.unwrap(),
                            widget.size.width,
                            widget.size.height,
                        );
                        let image: Image<_, C> = Image::new(&image_raw, Point::new(34, 0));
                        image
                            .draw(self.backend.draw_target())
                            .map_err(|_| MorphError::Backend("Could not draw image."))?;
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
            self.render = false;
        }

        // log("end render");
        self.backend.flush();

        Ok(())
    }

    /// Start and run the shell.
    pub fn run(&mut self) -> MorphResult<bool> {
        if !self.is_running {
            return Ok(false);
        }
        self.drain_events()?;
        self.build()?;
        self.update()?;
        self.draw()?;
        // log("end");

        Ok(true)
    }
}
