// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::sync::Arc;

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
    common::logger::init_logger();

    let db_file_path = common::dir::get_db_path_by_os().expect("Cannot get db path");
    common::dir::make_parent_dir_if_not_exists(&db_file_path).expect("Cannot make parent dir");

    let db = infra::core::seaorm::new(&db_file_path)
        .await
        .expect("Cannot connect to DB");
    let db = Arc::new(db);

    // マイグレーションを実行
    if env::var("APP_EXECUTION_MODE").unwrap_or_default() == "dev" {
        migration::migrator::Migrator::refresh(db.as_ref())
            .await
            .expect("Migration error");
    } else {
        migration::migrator::Migrator::up(db.as_ref(), None)
            .await
            .expect("Migration error");
    }

    let openai_client = infra::core::openai::OpenAIClient::new();
    let chat = infra::chat::OpenAIChat::new(openai_client);
    let settings_repository =
        infra::repository::prompt_manager::PromptManagerRepositoryImpl::new(db);
    let chat_usecase = usecase::chat::ChatUsecase::new(chat, settings_repository);
    controller::chat::Controller::init(chat_usecase);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            controller::handler::greet,
            controller::prompt_manager::save_prompt_manager,
            controller::prompt_manager::get_prompt_managers,
            controller::chat::post_chat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
