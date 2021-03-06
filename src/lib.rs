//! morph is a library to create user interfaces for embedded devices. It provides a shell as frame, startup and runner of
//! the main application. It also contains different widgets and layouts to build an user interface. morph could also run
//! in a browser with WebAssembly.
//! 
// #![cfg(not(target_arch = "arm"))]
// #![cfg_attr(feature = "no_std", no_std)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use platform::log;

#[cfg(target_arch = "wasm32")]
pub use canvas_display;

pub use embedded_graphics;

pub use heapless;

pub mod core;
pub mod geometry;
pub mod graphics;
pub mod platform;
pub mod shell;
pub mod theme;
pub mod prelude;
pub mod result;
pub mod widgets;

// todo: Inject static Theme type that provides style like button styles to read current properties dependent on the state.

// maybe Counter<Message, Theme> could be removed