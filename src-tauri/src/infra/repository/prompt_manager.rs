use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

use crate::common::errors::ApplicationError;
use crate::domain::prompt_manager::{PromptManagerModel, PromptManagerRepository};
use crate::infra::repository::entities::prelude::PromptManager;
use crate::infra::repository::entities::prompt_manager;

#[derive(Clone, Debug)]
pub struct PromptManagerRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

#[async_trait]
impl PromptManagerRepository for PromptManagerRepositoryImpl {
    async fn find_settings(&self) -> Result<Vec<PromptManagerModel>, ApplicationError> {
        let settings = PromptManager::find().all(self.db.as_ref()).await;
        match settings {
            Ok(setting) => Ok(setting
                .into_iter()
                .map(|s| PromptManagerModel {
                    id: s.id,
                    title: s.title,
                    api_type: s.api_type.parse().unwrap(), // enumがパースできないケースは無視する
                })
                .collect()),
            Err(err) => Err(ApplicationError::DBError(err)),
        }
    }

    async fn create_settings(&self, title: &str, api_type: &str) -> Result<i32, ApplicationError> {
        let setting = prompt_manager::ActiveModel {
            id: Default::default(),
            title: ActiveValue::Set(title.to_string()),
            api_type: ActiveValue::Set(api_type.to_string()),
        };
        let res = PromptManager::insert(setting)
            .exec(self.db.as_ref())
            .await
            .map_err(ApplicationError::DBError)?;
        Ok(res.last_insert_id)
    }
}

impl PromptManagerRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        PromptManagerRepositoryImpl { db }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
    use sea_orm_migration::MigratorTrait;

    use crate::domain::prompt_manager::{APIType, PromptManagerRepository};
    use crate::infra::repository::entities::prelude::PromptManager;
    use crate::infra::repository::entities::prompt_manager;
    use crate::infra::repository::prompt_manager::PromptManagerRepositoryImpl;
    use crate::{common, infra, migration};

    async fn setup_db(test_name: &str) -> Arc<DatabaseConnection> {
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

    #[tokio::test]
    async fn test_find_settings() {
        let db = setup_db("test_find_settings").await;
        let repo = PromptManagerRepositoryImpl::new(db.clone());

        let setting = prompt_manager::ActiveModel {
            id: Default::default(),
            title: ActiveValue::Set("test_title".to_string()),
            api_type: ActiveValue::Set(APIType::Chat.to_string()),
        };
        let _ = PromptManager::insert(setting)
            .exec(db.as_ref())
            .await
            .unwrap();

        let result = repo.find_settings().await;
        assert!(result.is_ok());
        let settings = result.unwrap();

        // assert
        assert_eq!(settings.len(), 1);
        assert_eq!(settings[0].title, "test_title");
        assert_eq!(settings[0].api_type, APIType::Chat);
    }

    #[tokio::test]
    async fn test_create_settings() {
        let db = setup_db("test_create_settings").await;
        let repo = PromptManagerRepositoryImpl::new(db.clone());

        // create_settingsメソッドを呼び出し
        let result = repo.create_settings("test_title", "test_api_type").await;
        assert!(result.is_ok());
        let id = result.unwrap();

        // assert
        let settings = PromptManager::find_by_id(id)
            .one(db.as_ref())
            .await
            .unwrap();
        let settings = settings.unwrap();
        assert_eq!(settings.title, "test_title");
        assert_eq!(settings.api_type, "test_api_type");
    }
}
