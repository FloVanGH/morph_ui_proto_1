pub use self::common::main_loop;
pub use self::graphics::*;

#[path = "../common/mod.rs"]
mod common;
mod graphics;

pub fn log(msg: &str) {
    println!("{}", msg);
}
