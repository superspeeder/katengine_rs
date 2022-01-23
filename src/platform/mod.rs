// #[cfg_attr(target_os = "linux", path = "linux.rs")]
// #[cfg_attr(target_os = "macos", path = "macos.rs")]
#[cfg_attr(windows, path = "windows.rs")]
pub mod internal;

