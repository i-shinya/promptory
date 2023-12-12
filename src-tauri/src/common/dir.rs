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

/// テスト用のDBパスを取得
pub fn get_test_home_path() -> Result<String, ApplicationError> {
    // テスト用
    let path = PathBuf::from("../data/test");
    return match path.to_str() {
        Some(path) => Ok(path.to_string()),
        None => Err(UnknownError("Cannot get app home path".to_string())),
    };
}

/// osを元にdbのパスを取得
pub fn get_db_path_by_os() -> Result<String, ApplicationError> {
    let app_home_dir = get_app_home_path()?;
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
pub fn make_parent_dir_if_not_exists(path: &str) -> Result<(), ApplicationError> {
    let paths = PathBuf::from(path);
    let parent_dir = paths.parent().unwrap();
    if !parent_dir.exists() {
        let res = fs::create_dir_all(parent_dir);
        if let Err(e) = res {
            return Err(ApplicationError::UnknownError(format!(
                "Cannot create parent dir '{}': {}",
                parent_dir
                    .to_str()
                    .ok_or_else(|| ApplicationError::UnknownError(
                        "Failed to convert path to string.".to_string()
                    ))?,
                e
            )));
        }
    }
    Ok(())
}
