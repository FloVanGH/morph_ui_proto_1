pub use self::platform::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native/mod.rs"]
mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
mod platform;