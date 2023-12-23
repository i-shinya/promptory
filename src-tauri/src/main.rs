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
        println!("dev mode: skip migration");
    } else {
        migration::migrator::Migrator::up(db.as_ref(), None)
            .await
            .expect("Migration error");
    }

    // infra層の初期化
    let openai_client = Arc::new(infra::core::openai::OpenAIClient::new());
    let chat = Arc::new(infra::chat::OpenAIChat::new(Arc::clone(&openai_client)));
    let prompt_manager_repository =
        Arc::new(infra::repository::prompt_manager::PromptManagerRepositoryImpl::new(db));
    // usecase層の初期化
    let chat_usecase = usecase::comparing_prompt::ChatUsecase::new(
        Arc::clone(&chat),
        Arc::clone(&prompt_manager_repository),
    );
    let prompt_manager_usecase =
        usecase::prompt_manager::PromptManagerUsecase::new(Arc::clone(&prompt_manager_repository));
    // controller層の初期化
    controller::comparing_prompt::Controller::init(chat_usecase);
    controller::prompt_manager::Controller::init(prompt_manager_usecase);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            controller::handler::greet,
            controller::prompt_manager::create_prompt_manager,
            controller::prompt_manager::update_prompt_manager,
            controller::prompt_manager::get_prompt_manager,
            controller::prompt_manager::get_all_prompt_managers,
            controller::prompt_manager::logical_delete_prompt_manager,
            controller::comparing_prompt::run_comparing_prompt,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
