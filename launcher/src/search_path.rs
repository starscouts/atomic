use std::path::PathBuf;

fn get_home() -> PathBuf {
    dirs::home_dir().unwrap_or(PathBuf::from("/"))
}

#[cfg(target_os = "macos")]
fn get_home_search() -> PathBuf {
    let mut path = get_home();
    path.push("/Library/Frameworks/AtomicRuntime.framework");
    path
}

#[cfg(target_os = "linux")]
fn get_home_search() -> PathBuf {
    let mut path = get_home();
    path.push("/.local/share/atomic-runtime");
    path
}

#[cfg(target_os = "windows")]
fn get_home_search() -> PathBuf {
    let mut path = get_home();
    path.push("\\AppData\\Local\\Atomic Runtime");
    path
}

#[cfg(target_os = "macos")]
pub fn get_search_paths() -> [PathBuf; 5] {
    [
        PathBuf::from("/Library/Frameworks/AtomicRuntime.framework"),
        PathBuf::from("/System/Library/Frameworks/AtomicRuntime.framework"),
        PathBuf::from("/usr/local/lib/atomic-runtime"),
        PathBuf::from("/opt/atomic-runtime"),
        get_home_search(),
    ]
}

#[cfg(target_os = "linux")]
pub fn get_search_paths() -> [PathBuf; 8] {
    [
        PathBuf::from("/usr/local/lib/atomic-runtime"),
        PathBuf::from("/usr/local/lib64/atomic-runtime"),
        PathBuf::from("/usr/lib/atomic-runtime"),
        PathBuf::from("/usr/lib64/atomic-runtime"),
        PathBuf::from("/lib/atomic-runtime"),
        PathBuf::from("/lib64/atomic-runtime"),
        PathBuf::from("/opt/atomic-runtime"),
        get_home_search(),
    ]
}

#[cfg(target_os = "windows")]
pub fn get_search_paths() -> [PathBuf; 29] {
    [
        PathBuf::from("C:\\Program Files\\Common Files\\Equestria.dev\\AtomicRuntime"),
        PathBuf::from("C:\\Program Files\\Common Files\\Equestria.dev\\Atomic Runtime"),
        PathBuf::from("C:\\Program Files (x86)\\Common Files\\Equestria.dev\\AtomicRuntime"),
        PathBuf::from("C:\\Program Files (x86)\\Common Files\\Equestria.dev\\Atomic Runtime"),
        PathBuf::from("C:\\Program Files (Arm)\\Common Files\\Equestria.dev\\AtomicRuntime"),
        PathBuf::from("C:\\Program Files (Arm)\\Common Files\\Equestria.dev\\Atomic Runtime"),
        PathBuf::from("C:\\Program Files\\Common Files\\AtomicRuntime"),
        PathBuf::from("C:\\Program Files\\Common Files\\Atomic Runtime"),
        PathBuf::from("C:\\Program Files (x86)\\Common Files\\AtomicRuntime"),
        PathBuf::from("C:\\Program Files (x86)\\Common Files\\Atomic Runtime"),
        PathBuf::from("C:\\Program Files (Arm)\\Common Files\\AtomicRuntime"),
        PathBuf::from("C:\\Program Files (Arm)\\Common Files\\Atomic Runtime"),
        PathBuf::from("C:\\Windows\\AtomicRuntime"),
        PathBuf::from("C:\\Windows\\Atomic Runtime"),
        PathBuf::from("C:\\Program Files\\Equestria.dev\\AtomicRuntime"),
        PathBuf::from("C:\\Program Files\\Equestria.dev\\Atomic Runtime"),
        PathBuf::from("C:\\Program Files (x86)\\Equestria.dev\\AtomicRuntime"),
        PathBuf::from("C:\\Program Files (x86)\\Equestria.dev\\Atomic Runtime"),
        PathBuf::from("C:\\Program Files (Arm)\\Equestria.dev\\AtomicRuntime"),
        PathBuf::from("C:\\Program Files (Arm)\\Equestria.dev\\Atomic Runtime"),
        PathBuf::from("C:\\Program Files\\AtomicRuntime"),
        PathBuf::from("C:\\Program Files\\Atomic Runtime"),
        PathBuf::from("C:\\Program Files (x86)\\AtomicRuntime"),
        PathBuf::from("C:\\Program Files (x86)\\Atomic Runtime"),
        PathBuf::from("C:\\Program Files (Arm)\\AtomicRuntime"),
        PathBuf::from("C:\\Program Files (Arm)\\Atomic Runtime"),
        PathBuf::from("C:\\ProgramData\\AtomicRuntime"),
        PathBuf::from("C:\\ProgramData\\Atomic Runtime"),
        get_home_search(),
    ]
}
