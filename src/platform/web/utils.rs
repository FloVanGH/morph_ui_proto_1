use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement, HtmlElement, Window};

use crate::result::*;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Gets the browser window.
pub fn window() -> MorphResult<Window> {
    web_sys::window().ok_or(MorphError::Backend(
        "utils::body: no global `windows` exists.",
    ))
}

/// Gets the document of the browser window.
pub fn document() -> MorphResult<Document> {
    window()?.document().ok_or(MorphError::Backend(
        "utils::body: should have a document on window.",
    ))
}

/// Gets the body of the browser document.
pub fn body() -> MorphResult<HtmlElement> {
    document()?.body().ok_or(MorphError::Backend(
        "utils::body: document should have a body.",
    ))
}

/// Gets a canvas by the given id or create it if it does not exists.
pub fn canvas(id: &str) -> MorphResult<HtmlCanvasElement> {
    let canvas = {
        if let Some(canvas) = document()?.get_element_by_id(id) {
            Some(canvas)
        } else {
            if let Ok(canvas) = document()?.create_element("canvas") {
                canvas.set_id(id);
                body()?.append_child(&canvas);
                Some(canvas)
            } else {
                None
            }
        }
    };

    canvas.map_or(
        Err(MorphError::Backend(
            "utils::canvas: Could not create canvas.",
        )),
        |c| {
            c.dyn_into::<HtmlCanvasElement>()
                .map_err(|_| MorphError::Backend("utils::canvas: Could not convert canvas."))
        },
    )
}

/// Creates a new canvas
pub fn create_canvas() -> MorphResult<HtmlCanvasElement> {
    document()?
        .create_element("canvas")
        .map_err(|_| MorphError::Backend(
            "utils::create_canvas: Could not create canvas.",
        ))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| MorphError::Backend("utils::create_canvas: Could not convert canvas."))
}
