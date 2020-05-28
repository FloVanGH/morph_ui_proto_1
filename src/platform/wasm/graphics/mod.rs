use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::ImageData;

use crate::{graphics::Color, result::*};
use embedded_graphics::{
    drawable::Pixel,
    geometry::Size,
    pixelcolor::{Rgb565, RgbColor},
    DrawTarget,
};

use super::utils;

#[derive(Clone, Debug, PartialEq)]
pub struct WasmDisplay {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    frame_buffer: Vec<(u8, u8, u8, u8)>,
}

impl WasmDisplay {
    pub fn new(width: u32, height: u32) -> MorphResult<Self> {
        let canvas = utils::canvas("morph_canvas")?;
        canvas.set_width(width);
        canvas.set_height(height);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .map_err(|_| {
                MorphError::Backend("WasmDisplay::new: Could not convert canvas context_2.")
            })?;

        let frame_buffer = vec![(0, 0, 0, 0); (width * height) as usize];

        Ok(WasmDisplay {
            canvas,
            context,
            frame_buffer,
        })
    }
}

impl DrawTarget<Rgb565> for WasmDisplay {
    type Error = MorphError;

    fn draw_pixel(&mut self, pixel: Pixel<Rgb565>) -> Result<(), Self::Error> {
        let index = pixel.0.y * self.canvas.width() as i32 + pixel.0.x;

        if index < 0 || index as usize > self.frame_buffer.len() {
            return Err(MorphError::Backend(
                "WasmDisplay::draw_pixel: Pixel coordinate is out of buffer bounds.",
            ));
        }

        self.frame_buffer[(pixel.0.y * self.canvas.width() as i32 + pixel.0.x) as usize] = (pixel.1.r(), pixel.1.g(), pixel.1.b(), 255);

        let data = web_sys::ImageData::new_with_u8_clamped_array(
            Clamped(&mut vec![pixel.1.r(), pixel.1.g(), pixel.1.b(), 250]),
            1,
        )
        .expect("RenderContext.draw_image: Could not create image data.");

        self.context
            .put_image_data(&data, pixel.0.x as f64, pixel.0.y as f64);

        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(self.canvas.width(), self.canvas.height())
    }
}
