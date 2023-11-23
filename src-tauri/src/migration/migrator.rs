pub use sea_orm_migration::prelude::*;

mod m000001_init;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // migrationファイルを追加したらここにも追加する
            Box::new(m000001_init::Migration),
        ]
    }
}
