use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

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
    async fn find_all(&self) -> Result<Vec<PromptManagerModel>, ApplicationError> {
        let settings = PromptManager::find().all(self.db.as_ref()).await;
        match settings {
            Ok(setting) => Ok({
                setting
                    .into_iter()
                    .map(|s| PromptManagerModel {
                        id: s.id,
                        title: s.title,
                        api_type: s.api_type.map(|a| a.parse().unwrap()),
                    })
                    .collect()
            }),
            Err(err) => Err(ApplicationError::DBError(err)),
        }
    }

    async fn create(&self, title: &str) -> Result<i32, ApplicationError> {
        let prompt_manager = prompt_manager::ActiveModel {
            id: Default::default(),
            title: ActiveValue::Set(title.to_string()),
            api_type: ActiveValue::Set(None),   // 初期値はNone
            deleted_at: ActiveValue::Set(None), // 初期値はNone
        };
        let res = PromptManager::insert(prompt_manager)
            .exec(self.db.as_ref())
            .await
            .map_err(ApplicationError::DBError)?;
        Ok(res.last_insert_id)
    }

    async fn logical_delete(&self, id: i32) -> Result<i32, ApplicationError> {
        let prompt_manager = PromptManager::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(ApplicationError::DBError)?;
        if prompt_manager.is_none() {
            return Err(ApplicationError::EmptyResult);
        }
        let mut prompt_manager: prompt_manager::ActiveModel = prompt_manager.unwrap().into();
        prompt_manager.deleted_at = ActiveValue::Set(Some(chrono::Utc::now().to_string()));
        let res = prompt_manager
            .update(self.db.as_ref())
            .await
            .map_err(ApplicationError::DBError)?;
        Ok(res.id)
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

        let prompt_manager = prompt_manager::ActiveModel {
            id: Default::default(),
            title: ActiveValue::Set("test_title".to_string()),
            api_type: ActiveValue::Set(Option::from(APIType::Chat.to_string())),
            deleted_at: ActiveValue::Set(None),
        };
        let _ = PromptManager::insert(prompt_manager)
            .exec(db.as_ref())
            .await
            .expect("Failed to insert prompt manager");

        let result = repo.find_all().await;
        assert!(result.is_ok());
        let settings = result.unwrap();

        // assert
        assert_eq!(settings.len(), 1);
        assert_eq!(settings[0].title, "test_title");
        assert_eq!(settings[0].api_type, Some(APIType::Chat));
    }

    #[tokio::test]
    async fn test_create_settings() {
        let db = setup_db("test_create_settings").await;
        let repo = PromptManagerRepositoryImpl::new(db.clone());

        // create_settingsメソッドを呼び出し
        let result = repo.create("test_title").await;
        assert!(result.is_ok());
        let id = result.unwrap();

        // assert
        let prompt_managers = PromptManager::find_by_id(id)
            .one(db.as_ref())
            .await
            .expect("Failed to insert prompt manager");
        let settings = prompt_managers.unwrap();
        assert_eq!(settings.title, "test_title");
        assert_eq!(settings.api_type, None);
    }

    #[tokio::test]
    async fn test_logical_delete() {
        let db = setup_db("test_logical_delete").await;
        let repo = PromptManagerRepositoryImpl::new(db.clone());

        // まずは設定を作成
        let prompt_manager = prompt_manager::ActiveModel {
            id: Default::default(),
            title: ActiveValue::Set("test_title".to_string()),
            api_type: ActiveValue::Set(Option::from(APIType::Chat.to_string())),
            deleted_at: ActiveValue::Set(None),
        };
        let _ = PromptManager::insert(prompt_manager)
            .exec(db.as_ref())
            .await
            .expect("Failed to insert prompt manager");

        // 作成した設定を削除
        let result = repo.logical_delete(1).await;
        assert!(result.is_ok());

        // 削除した設定が存在し、deleted_atがNoneでないことを確認
        let prompt_managers = PromptManager::find_by_id(1)
            .one(db.as_ref())
            .await
            .expect("Failed to fetch prompt manager");
        assert!(prompt_managers.is_some());
        assert!(prompt_managers.unwrap().deleted_at.is_some());
    }

    #[tokio::test]
    async fn test_logical_delete_error() {
        let db = setup_db("test_logical_delete_error").await;
        let repo = PromptManagerRepositoryImpl::new(db.clone());

        // 存在しないIDでlogical_deleteメソッドを呼び出し
        let result = repo.logical_delete(9999).await;
        assert!(result.is_err());
    }
}
