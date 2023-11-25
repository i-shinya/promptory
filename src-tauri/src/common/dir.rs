use std::path::PathBuf;
use std::{env, fs};

/// osを元にappデータの保存パスを取得
pub fn get_app_home_path() -> String {
    // 開発モードの場合はproject_root/app
    if env::var("MODE").unwrap_or_default() == "dev" {
        println!("dev mode");
        let path = PathBuf::from("../data");
        return path.to_str().unwrap().to_string();
    }

    let home_dir = env::var("HOME")
        .or(env::var("USERPROFILE"))
        .expect("Cannot find home directory");

    let mut app_dir = PathBuf::from(home_dir);
    if cfg!(target_os = "windows") {
        app_dir.push("AppData\\Roaming\\darq\\promptory");
    } else if cfg!(target_os = "macos") {
        app_dir.push("Library/Application Support/darq/promptory");
    } else {
        // linux
        app_dir.push(".config/darq/promptory");
    }
    app_dir.to_str().unwrap().to_string()
}

/// osを元にdbのパスを取得
pub fn get_db_path_by_os() -> String {
    let app_home_dir = get_app_home_path();
    let mut app_dir = PathBuf::from(app_home_dir);
    if cfg!(target_os = "windows") {
        app_dir.push("db\\database.db");
    } else {
        // mac or linux
        app_dir.push("db/database.db");
    }
    app_dir.to_str().unwrap().to_string()
}

/// ファイルの親ディレクトリが存在しない場合は再起的に作成する
pub fn make_parent_dir_if_not_exists(path: &str) {
    let paths = PathBuf::from(path);
    let parent_dir = paths.parent().unwrap();
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).unwrap();
    }
}
