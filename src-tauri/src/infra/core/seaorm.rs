use sea_orm::DatabaseConnection;

pub async fn new(path: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    // database.rcがない場合は?mode=rwcが必要そう[参考](https://github.com/SeaQL/sea-orm/discussions/283#discussioncomment-1564939)
    let db_path = format!("sqlite:{}?mode=rwc", path);
    println!("db_path: {}", db_path);
    sea_orm::Database::connect(db_path).await
}
