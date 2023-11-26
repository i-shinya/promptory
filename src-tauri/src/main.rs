// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use sea_orm_migration::prelude::*;

mod common;
mod controller;
mod domain;
mod infra;
mod migration;
mod usecase;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_file_path = common::dir::get_db_path_by_os();
    if db_file_path.is_err() {
        panic!("Cannot get db path");
    }
    let db_file_path = db_file_path.unwrap();
    common::dir::make_parent_dir_if_not_exists(&db_file_path);

    // database.rcがない場合は?mode=rwcが必要そう[参考](https://github.com/SeaQL/sea-orm/discussions/283#discussioncomment-1564939)
    let db_path = format!("sqlite:{}?mode=rwc", &db_file_path);
    let db = sea_orm::Database::connect(db_path).await.unwrap();

    // マイグレーションを実行
    let _schema_manager = SchemaManager::new(&db);
    let res = migration::migrator::Migrator::refresh(&db).await;
    if let Err(err) = res {
        panic!("Migration error: {}", err);
    }

    let openai_client = infra::core::openai::new_client();
    let chat = infra::chat::new(openai_client);
    let chat_usecase = usecase::chat::new(chat);
    controller::chat::init(chat_usecase);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            controller::handler::greet,
            controller::chat::post_chat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
