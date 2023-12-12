use std::sync::Arc;

use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;

use crate::{common, infra, migration};

pub async fn setup_db(test_name: &str) -> Arc<DatabaseConnection> {
    let db_file_path = common::dir::get_test_home_path().expect("Cannot get db path");
    let db_file_path = format!("{}/{}", db_file_path, test_name);
    common::dir::make_parent_dir_if_not_exists(&db_file_path).expect("Cannot make parent dir");

    let db = infra::core::seaorm::new(&db_file_path)
        .await
        .expect("Cannot connect to DB");
    let db = Arc::new(db);

    migration::migrator::Migrator::refresh(db.as_ref())
        .await
        .expect("Migration error");
    db
}
