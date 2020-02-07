pub use self::common::main_loop;

#[path = "../common/mod.rs"]
mod common;

/// Outputs a message to the console.
pub fn log(msg: &str) {
    println!("{}", msg);
}

