pub use common::*;
pub use self::raqote::*;

#[path = "../common/mod.rs"]
mod common;
mod raqote;

pub fn log(msg: &str) {
    println!("{}", msg);
}