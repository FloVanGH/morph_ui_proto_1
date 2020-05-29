pub use common::*;

#[path = "../common/mod.rs"]
mod common;

pub fn log(msg: &str) {
    println!("{}", msg);
}