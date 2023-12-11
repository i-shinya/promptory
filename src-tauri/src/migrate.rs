use std::path::PathBuf;

use sea_orm_migration::MigratorTrait;

mod migration;

/// ローカル環境で使用するマイグレーションスクリプト
#[tokio::main]
async fn main() {
    let mut path = PathBuf::from("../data");
    path.push("db/database.db");
    let path = path.to_str().unwrap();

    let db_path = format!("sqlite:{}?mode=rwc", path);
    let db = sea_orm::Database::connect(db_path).await;
    let db = db.unwrap();

    migration::migrator::Migrator::refresh(&db)
        .await
        .expect("Migration error");
}
