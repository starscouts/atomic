use std::ffi::OsStr;
use std::path::PathBuf;
use crate::AppDataError::NotFound;

pub mod constants;

pub enum AppData {
    Folder(PathBuf),
    Asar(PathBuf)
}

pub enum AppDataError {
    NotFound(PathBuf, PathBuf)
}

impl AppData {
    pub fn get_path(&self) -> &OsStr {
        match self {
            AppData::Folder(path) => path.as_ref(),
            AppData::Asar(path) => path.as_ref()
        }
    }

    pub fn locate_app(work_dir: &PathBuf) -> Result<AppData, AppDataError> {
        let mut app_path_folder = PathBuf::from(work_dir);
        app_path_folder.push(constants::APP_FOLDER_PATH);

        let mut app_path_asar = PathBuf::from(work_dir);
        app_path_asar.push(constants::APP_ASAR_PATH);

        if app_path_folder.exists() {
            Ok(AppData::Folder(app_path_folder))
        } else if app_path_asar.exists() {
            Ok(AppData::Asar(app_path_asar))
        } else {
            Err(NotFound(app_path_folder, app_path_asar))
        }
    }
}
