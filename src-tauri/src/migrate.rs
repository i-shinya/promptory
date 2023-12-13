use std::fs;
use std::path::PathBuf;

use sea_orm_migration::MigratorTrait;

mod migration;

/// ローカル環境で使用するマイグレーションスクリプト
#[tokio::main]
async fn main() {
    let mut path = PathBuf::from("../data");
    path.push("db/database.db");
    let path = path.to_str().unwrap();
    make_parent_dir_if_not_exists(&path);

    let db_path = format!("sqlite:{}?mode=rwc", path);
    let db = sea_orm::Database::connect(db_path).await;
    let db = db.unwrap();

    migration::migrator::Migrator::refresh(&db)
        .await
        .expect("Migration error");
}

/// ファイルの親ディレクトリが存在しない場合は再起的に作成する
pub fn make_parent_dir_if_not_exists(path: &str) {
    let paths = PathBuf::from(path);
    let parent_dir = paths.parent().unwrap();
    if !parent_dir.exists() {
        let res = fs::create_dir_all(parent_dir);
        res.unwrap_or_else(|_| {
            panic!(
                "Cannot create parent dir '{}'",
                parent_dir.to_str().unwrap()
            )
        });
    }
}
