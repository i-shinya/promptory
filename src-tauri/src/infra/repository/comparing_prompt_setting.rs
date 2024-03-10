use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{
    ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, LoaderTrait, ModelTrait,
    QueryFilter, QuerySelect, RelationTrait, TransactionTrait,
};

use crate::common::errors::ApplicationError;
use crate::domain::comparing_prompt::{
    ComparingPromptSettingModel, ComparingPromptSettingRepository,
    ComparingPromptSettingVersionModel,
};
use crate::infra::repository::entities::prelude::{
    ComparingPromptSettingVersions, ComparingPromptSettings,
};
use crate::infra::repository::entities::{
    comparing_prompt_setting_versions, comparing_prompt_settings,
};

#[derive(Clone, Debug)]
pub struct ComparingPromptSettingRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

#[async_trait]
impl ComparingPromptSettingRepository for ComparingPromptSettingRepositoryImpl {
    async fn find_comparing_prompt_setting_by_id(
        &self,
        id: i32,
    ) -> Result<ComparingPromptSettingModel, ApplicationError> {
        let res = ComparingPromptSettings::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(ApplicationError::DBError)?;

        if res.is_none() {
            return Err(ApplicationError::EmptyResult);
        }
        let res = res.unwrap();

        let version = res
            .find_related(comparing_prompt_setting_versions::Entity)
            .filter(comparing_prompt_setting_versions::Column::Version.eq(res.current_version))
            .one(self.db.as_ref())
            .await
            .map_err(ApplicationError::DBError)?;

        if version.is_none() {
            return Err(ApplicationError::EmptyResult);
        }
        let version = version.unwrap();

        Ok(ComparingPromptSettingModel {
            id: res.id,
            manager_id: res.manager_id,
            current_version: res.current_version,
            versions: vec![ComparingPromptSettingVersionModel {
                id: version.id,
                setting_id: version.setting_id,
                version: version.version,
                system_prompt: version.system_prompt,
            }],
        })
    }

    async fn find_all_comparing_prompt_settings_by_manager_id(
        &self,
        manager_id: i32,
    ) -> Result<Vec<ComparingPromptSettingModel>, ApplicationError> {
        let res = ComparingPromptSettings::find()
            .column(comparing_prompt_setting_versions::Column::SystemPrompt)
            .filter(
                Condition::all()
                    .add(comparing_prompt_settings::Column::ManagerId.eq(manager_id))
                    .add(comparing_prompt_settings::Column::DeletedAt.is_null()),
            )
            .find_with_related(comparing_prompt_setting_versions::Entity)
            .all(self.db.as_ref())
            .await
            .map_err(ApplicationError::DBError)?;

        Ok({
            res.into_iter()
                .map(|prompt_manager| ComparingPromptSettingModel {
                    id: prompt_manager.0.id,
                    manager_id: prompt_manager.0.manager_id,
                    current_version: prompt_manager.0.current_version,
                    versions: prompt_manager
                        .1
                        .into_iter()
                        .map(|version| ComparingPromptSettingVersionModel {
                            id: version.id,
                            setting_id: version.setting_id,
                            version: version.version,
                            system_prompt: version.system_prompt,
                        })
                        .collect(),
                })
                .collect()
        })
    }

    async fn create_comparing_prompt_setting(
        &self,
        manager_id: i32,
    ) -> Result<i32, ApplicationError> {
        let prompt_manager = comparing_prompt_settings::ActiveModel {
            id: Default::default(),
            manager_id: ActiveValue::Set(manager_id),
            current_version: ActiveValue::Set(1),
            deleted_at: ActiveValue::Set(None),
        };
        let res = ComparingPromptSettings::insert(prompt_manager)
            .exec(self.db.as_ref())
            .await
            .map_err(ApplicationError::DBError)?;

        // バージョン１のComparingPromptSettingVersionを作成
        let version = comparing_prompt_setting_versions::ActiveModel {
            id: Default::default(),
            setting_id: ActiveValue::Set(res.last_insert_id),
            version: ActiveValue::Set(1),
            system_prompt: ActiveValue::Set("".to_string()),
        };
        let _ = ComparingPromptSettingVersions::insert(version)
            .exec(self.db.as_ref())
            .await
            .map_err(ApplicationError::DBError)?;
        Ok(res.last_insert_id)
    }
}

impl ComparingPromptSettingRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ComparingPromptSettingRepositoryImpl { db }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::thelper::db::setup_db;
    use crate::infra::repository::entities::prelude::{ComparingPromptManager, PromptManager};
    use crate::infra::repository::entities::{comparing_prompt_manager, prompt_manager};

    use super::*;

    async fn seed_prompt_manager(db: Arc<DatabaseConnection>) -> i32 {
        let prompt_manager = prompt_manager::ActiveModel {
            id: Default::default(),
            title: ActiveValue::Set("test_title".to_string()),
            action_type: ActiveValue::Set(None),
            api_type: ActiveValue::Set(None),
            deleted_at: ActiveValue::Set(None),
        };
        let inserted_prompt_manager = PromptManager::insert(prompt_manager)
            .exec(db.as_ref())
            .await
            .expect("Failed to insert prompt manager");
        inserted_prompt_manager.last_insert_id
    }

    async fn seed_comparing_prompt_manager(db: Arc<DatabaseConnection>, manager_id: i32) -> i32 {
        let comparing_prompt_manager = comparing_prompt_manager::ActiveModel {
            manager_id: ActiveValue::Set(manager_id),
        };
        let inserted_comparing_prompt_manager =
            ComparingPromptManager::insert(comparing_prompt_manager)
                .exec(db.as_ref())
                .await
                .expect("Failed to insert comparing_prompt_manager");
        inserted_comparing_prompt_manager.last_insert_id
    }

    #[tokio::test]
    async fn test_find_comparing_prompt_setting_by_id() {
        let db = setup_db("test_find_comparing_prompt_setting_by_id").await;
        let repository = ComparingPromptSettingRepositoryImpl::new(db.clone());

        // 事前データ
        let manager_id = seed_prompt_manager(Arc::clone(&db)).await;
        let _ = seed_comparing_prompt_manager(Arc::clone(&db), manager_id).await;

        let comparing_prompt_setting = comparing_prompt_settings::ActiveModel {
            id: Default::default(),
            manager_id: ActiveValue::Set(manager_id),
            current_version: ActiveValue::Set(1),
            deleted_at: ActiveValue::Set(None),
        };
        let inserted_comparing_prompt_setting =
            ComparingPromptSettings::insert(comparing_prompt_setting)
                .exec(db.as_ref())
                .await
                .expect("Failed to insert comparing_prompt_setting");
        let id = inserted_comparing_prompt_setting.last_insert_id;

        let comparing_prompt_setting_version = comparing_prompt_setting_versions::ActiveModel {
            id: Default::default(),
            setting_id: ActiveValue::Set(id),
            version: ActiveValue::Set(1),
            system_prompt: ActiveValue::Set("".to_string()),
        };
        let _ = ComparingPromptSettingVersions::insert(comparing_prompt_setting_version)
            .exec(db.as_ref())
            .await
            .expect("Failed to insert comparing_prompt_setting_version");

        // テスト対象のメソッドを呼び出し
        let result = repository.find_comparing_prompt_setting_by_id(id).await;

        // assert
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.id, id);
        assert_eq!(result.manager_id, manager_id);
        assert_eq!(result.current_version, 1);
        assert_eq!(result.versions.len(), 1);
        assert_eq!(result.versions[0].system_prompt, "");
    }

    #[tokio::test]
    async fn test_find_all_comparing_prompt_settings_by_manager_id() {
        let db = setup_db("test_find_all_comparing_prompt_settings_by_manager_id").await;
        let repository = ComparingPromptSettingRepositoryImpl::new(db.clone());

        // 事前データ
        let manager_id = seed_prompt_manager(Arc::clone(&db)).await;
        let _ = seed_comparing_prompt_manager(Arc::clone(&db), manager_id).await;

        let comparing_prompt_setting = comparing_prompt_settings::ActiveModel {
            id: Default::default(),
            manager_id: ActiveValue::Set(manager_id),
            current_version: ActiveValue::Set(1),
            deleted_at: ActiveValue::Set(None),
        };
        let inserted_comparing_prompt_setting =
            ComparingPromptSettings::insert(comparing_prompt_setting)
                .exec(db.as_ref())
                .await
                .expect("Failed to insert comparing_prompt_setting");
        let id = inserted_comparing_prompt_setting.last_insert_id;

        let comparing_prompt_setting_version = comparing_prompt_setting_versions::ActiveModel {
            id: Default::default(),
            setting_id: ActiveValue::Set(id),
            version: ActiveValue::Set(1),
            system_prompt: ActiveValue::Set("".to_string()),
        };
        let _ = ComparingPromptSettingVersions::insert(comparing_prompt_setting_version)
            .exec(db.as_ref())
            .await
            .expect("Failed to insert comparing_prompt_setting_version");

        // テスト対象のメソッドを呼び出し
        let result = repository
            .find_all_comparing_prompt_settings_by_manager_id(manager_id)
            .await;

        // assert
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, id);
        assert_eq!(result[0].manager_id, manager_id);
        assert_eq!(result[0].current_version, 1);
        assert_eq!(result[0].versions.len(), 1);
        assert_eq!(result[0].versions[0].system_prompt, "");
    }

    #[tokio::test]
    async fn test_create_comparing_prompt_setting() {
        let db = setup_db("test_create_comparing_prompt_setting").await;
        let repository = ComparingPromptSettingRepositoryImpl::new(db.clone());

        // 事前データ
        let manager_id = seed_prompt_manager(Arc::clone(&db)).await;
        let _ = seed_comparing_prompt_manager(Arc::clone(&db), manager_id).await;

        // テスト対象のメソッドを呼び出し
        let result = repository.create_comparing_prompt_setting(manager_id).await;

        // assert
        assert!(result.is_ok());
        let new_id = result.unwrap();
        let new_item = repository
            .find_comparing_prompt_setting_by_id(new_id)
            .await
            .unwrap();
        assert_eq!(new_item.id, new_id);
        assert_eq!(new_item.manager_id, manager_id);
        assert_eq!(new_item.current_version, 1);

        // バージョンテーブルも作成されていることを確認
        let version_result = ComparingPromptSettingVersions::find()
            .filter(comparing_prompt_setting_versions::Column::SettingId.eq(new_id))
            .one(db.as_ref())
            .await
            .expect("Failed to fetch comparing_prompt_setting_version");
        assert!(version_result.is_some());
        let version = version_result.unwrap();
        assert_eq!(version.setting_id, new_id);
        assert_eq!(version.version, 1);
    }
}
