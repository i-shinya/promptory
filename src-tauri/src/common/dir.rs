use std::path::PathBuf;
use std::{env, fs};

use crate::common::errors::ApplicationError;
use crate::common::errors::ApplicationError::UnknownError;

/// osを元にappデータの保存パスを取得
pub fn get_app_home_path() -> Result<String, ApplicationError> {
    // 開発モードの場合はproject_root/app
    if env::var("APP_EXECUTION_MODE").unwrap_or_default() == "dev" {
        println!("dev mode");
        let path = PathBuf::from("../data");
        return match path.to_str() {
            Some(path) => Ok(path.to_string()),
            None => Err(UnknownError("Cannot get app home path".to_string())),
        };
    }

    let home_dir = env::var("HOME")
        .or(env::var("USERPROFILE"))
        .map_err(|_| "Cannot find home directory");
    if home_dir.is_err() {
        return Err(UnknownError(home_dir.err().unwrap().to_string()));
    }

    let home_dir = home_dir.unwrap();
    let mut app_dir = PathBuf::from(home_dir);
    if cfg!(target_os = "windows") {
        app_dir.push("AppData\\Roaming\\darq\\promptory");
    } else if cfg!(target_os = "macos") {
        app_dir.push("Library/Application Support/darq/promptory");
    } else {
        // linux
        app_dir.push(".config/darq/promptory");
    }
    return match app_dir.to_str() {
        Some(path) => Ok(path.to_string()),
        None => Err(UnknownError("Cannot get app home path".to_string())),
    };
}

/// osを元にdbのパスを取得
pub fn get_db_path_by_os() -> Result<String, ApplicationError> {
    let app_home_dir = get_app_home_path();
    if app_home_dir.is_err() {
        return Err(app_home_dir.err().unwrap());
    }
    let app_home_dir = app_home_dir.unwrap();
    let mut app_dir = PathBuf::from(app_home_dir);
    if cfg!(target_os = "windows") {
        app_dir.push("db\\database.db");
    } else {
        // mac or linux
        app_dir.push("db/database.db");
    }
    return match app_dir.to_str() {
        Some(path) => Ok(path.to_string()),
        None => Err(UnknownError("Cannot get db path".to_string())),
    };
}

/// ファイルの親ディレクトリが存在しない場合は再起的に作成する
pub fn make_parent_dir_if_not_exists(path: &str) {
    let paths = PathBuf::from(path);
    let parent_dir = paths.parent().unwrap();
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }
}
