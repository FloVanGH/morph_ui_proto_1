//! morph is a library to create user interfaces for embedded devices. It provides a shell as frame, startup and runner of
//! the main application. It also contains different widgets and layouts to build an user interface. morph could also run
//! in a browser with WebAssembly.
//! 

#![no_std]

pub mod platform;
pub mod shell;
pub mod prelude;