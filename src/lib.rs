#[cfg(not(target_os = "windows"))]
mod unix;
#[cfg(not(target_os = "windows"))]
pub use unix::*;

#[cfg(target_os = "windows")]
pub  mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;