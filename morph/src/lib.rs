//! morph is a library to create user interfaces for embedded devices. It provides a shell as frame, startup and runner of
//! the main application. It also contains different widgets and layouts to build an user interface. morph could also run
//! in a browser with WebAssembly.
//!

pub use embedded_graphics;

pub mod core;
pub mod geometry;
pub mod graphics;
pub mod prelude;
pub mod result;
pub mod theme;
pub mod widgets;

// todo: Inject static Theme type that provides style like button styles to read current properties dependent on the state.

// maybe Counter<Message, Theme> could be removed
