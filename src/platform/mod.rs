pub use self::platform::*;

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[path = "cortex_m/mod.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "linux_embedded/mod.rs"]
mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
mod platform;