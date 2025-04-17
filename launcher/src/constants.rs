#[cfg(target_os = "macos")]
pub const BINARY_PATH: &str = "Contents/MacOS/Atomic";

#[cfg(target_os = "linux")]
pub const BINARY_PATH: &str = "atomic";

#[cfg(target_os = "windows")]
pub const BINARY_PATH: &str = "atomic.exe";

#[cfg(target_os = "macos")]
pub const APP_FOLDER_PATH: &str = "../Resources/app";

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub const APP_FOLDER_PATH: &str = "./resources/app";

#[cfg(target_os = "macos")]
pub const APP_ASAR_PATH: &str = "../Resources/app.asar";

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub const APP_ASAR_PATH: &str = "./resources/app.asar";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
