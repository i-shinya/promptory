// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs;
use std::path::PathBuf;

use dotenv::dotenv;
use sea_orm_migration::prelude::*;

mod controller;
mod domain;
mod infra;
mod migration;
mod usecase;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("MODE: {}", env::var("MODE").unwrap_or_default()

    let db_file_path = get_db_path_by_os();
    make_parent_dir_if_not_exists(&db_file_path);

    // database.rcがない場合は?mode=rwcが必要そう[参考](https://github.com/SeaQL/sea-orm/discussions/283#discussioncomment-1564939)
    let db_path = format!("sqlite:{}?mode=rwc", &db_file_path);
    let db = sea_orm::Database::connect(db_path).await.unwrap();

    // マイグレーションを実行
    let _schema_manager = SchemaManager::new(&db);
    let res = migration::migrator::Migrator::refresh(&db).await;
    if let Err(err) = res {
        println!("Migration error: {}", err);
        panic!("Migration error: {}", err)
    }

    let _openai_client = infra::core::openai::new_client();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![controller::handler::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// osを元にdbのパスを取得
fn get_db_path_by_os() -> String {
    let home_dir = env::var("HOME")
        .or(env::var("USERPROFILE"))
        .expect("Cannot find home directory");

    // 開発モードの場合はカレントディレクトリにdbを作成する
    if env::var("MODE").unwrap_or_default() == "dev" {
        println!("dev mode");
        // TODO あとで判定を修正
        return "../db/database.db".to_string();
    }

    let mut app_dir = PathBuf::from(home_dir);

    if cfg!(target_os = "windows") {
        app_dir.push("AppData\\Roaming\\darq\\promptory\\db\\database.db");
    } else if cfg!(target_os = "macos") {
        app_dir.push("Library/Application Support/darq/promptory/db/database.db");
    } else {
        app_dir.push(".config/darq/promptory/db/database.db");
    }
    app_dir.to_str().unwrap().to_string()
}

/// ファイルの親ディレクトリが存在しない場合は再起的に作成する
fn make_parent_dir_if_not_exists(path: &str) {
    let paths = PathBuf::from(path);
    let parent_dir = paths.parent().unwrap();
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).unwrap();
    }
}
