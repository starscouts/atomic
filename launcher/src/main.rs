#![windows_subsystem = "windows"]

use std::path::PathBuf;
use std::process;
use launcher::{AppData, AppDataError, constants};

mod search_path;

fn show_system_info() {
    println!(
        "atomic-launcher: Starting atomic-launcher {} on {} ({})",
        constants::VERSION,
        std::env::consts::OS,
        std::env::consts::ARCH
    );

    println!(
        "atomic-launcher: Compiled against rustc {}",
        rustc_version_runtime::version()
    );
}

fn is_valid_path(mut path: PathBuf) -> Option<PathBuf> {
    path.push(constants::BINARY_PATH);
    let as_path = path.as_path();

    if as_path.is_file() {
        Some(path)
    } else {
        None
    }
}

fn main() {
    show_system_info();
    let work_dir = get_working_directory();

    println!(
        "atomic-launcher: Current directory: {}",
        work_dir.to_str().unwrap()
    );

    let app_data = AppData::locate_app(&work_dir);
    let runtime_path = locate_runtime();
    launch_app(runtime_path, app_data)
}

fn get_working_directory() -> PathBuf {
    let exec_path = std::env::current_exe().ok();
    exec_path.as_ref()
        .and_then(|p| p.parent())
        .unwrap()
        .to_owned()
}

fn locate_runtime() -> Option<PathBuf> {
    let runtime_search_paths = search_path::get_search_paths();
    let runtime_search_paths_string = runtime_search_paths
        .iter()
        .map(|i| i.to_str().unwrap())
        .collect::<Vec<&str>>()
        .join(", ");

    println!(
        "atomic-launcher: Looking for atomic-runtime in: {}",
        runtime_search_paths_string
    );

    runtime_search_paths.into_iter()
        .filter_map(is_valid_path)
        .next()
}

fn launch_app(runtime_path: Option<PathBuf>, app_data: Result<AppData, AppDataError>) {
    if let Some(path) = runtime_path {
        println!(
            "atomic-launcher: Found atomic-runtime at {}",
            path.to_str().unwrap()
        );
        let mut cmd = process::Command::new(path);
        cmd.env("ATOMIC_LAUNCHER_VERSION", constants::VERSION);
        println!("atomic-launcher: Executing: {:?}", cmd);

        match app_data {
            Ok(app_data) => {
                cmd.arg(app_data.get_path());
            }
            Err(AppDataError::NotFound(app_path_folder, app_path_asar)) => {
                println!(
                    "atomic-launcher: Warning: Could not find Electron-compatible app.\
                    Looked in {} and {}. The default atomic-runtime application will be opened instead.",
                    app_path_folder.to_str().unwrap(),
                    app_path_asar.to_str().unwrap()
                );
            }
        }

        if let Ok(status) = cmd.status() {
            process::exit(status.code().unwrap_or(255));
        } else {
            eprintln!("atomic-launcher: Application failed to start.");
            process::exit(3);
        }
    } else {
        eprintln!("atomic-launcher: Could not find a valid atomic-runtime path. Aborting.");
        process::exit(2);
    }
}
