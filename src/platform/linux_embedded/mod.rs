pub use self::common::main_loop;

#[path = "../common/mod.rs"]
mod common;

pub mod render_context;

/// Outputs a message to the console.
pub fn log(msg: &str) {
    println!("{}", msg);
}

