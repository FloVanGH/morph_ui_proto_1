use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

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

/// Gets the body of the browser document.
pub fn body() -> MorphResult<HtmlElement> {
    let window = web_sys::window();

    if window.is_none() {
        return Err(MorphError::Backend(
            "utils::body: no global `windows` exists.",
        ));
    }

    let document = window.unwrap().document();

    if document.is_none() {
        return Err(MorphError::Backend(
            "utils::body: should have a document on window.",
        ));
    }

    let body = document.unwrap().body();

    if body.is_none() {
        return Err(MorphError::Backend(
            "utils::body: document should have a body.",
        ));
    }

    Ok(body.unwrap())
}
