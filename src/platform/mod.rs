pub use self::platform::*;

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[path = "cortex_m/mod.rs"]
mod platform;

#[cfg(all(not(target_arch = "wasm32"), not(target_arch = "arm")))]
#[path = "desktop/mod.rs"]
mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "wasm/mod.rs"]
mod platform;