use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{
    ActiveValue, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, QuerySelect,
    RelationTrait, TransactionTrait,
};

use crate::common::errors::ApplicationError;
use crate::domain::comparing_prompt::{
    ComparingPromptRunRepository, ComparingPromptSettingRunModel,
};
use crate::infra::repository::entities::comparing_prompt_runs;
use crate::infra::repository::entities::prelude::ComparingPromptRuns;

#[derive(Clone, Debug)]
pub struct ComparingPromptRunRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

#[async_trait]
impl ComparingPromptRunRepository for ComparingPromptRunRepositoryImpl {
    async fn find_comparing_prompt_run_by_id(
        &self,
        id: i32,
    ) -> Result<ComparingPromptSettingRunModel, ApplicationError> {
        let comparing_prompt_run = ComparingPromptRuns::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(ApplicationError::DBError)?;
        let comparing_prompt_run = comparing_prompt_run.ok_or(ApplicationError::EmptyResult)?;
        Ok(ComparingPromptSettingRunModel {
            id: comparing_prompt_run.id,
            manager_id: comparing_prompt_run.manager_id,
            user_prompt: comparing_prompt_run.user_prompt,
            provider_type: comparing_prompt_run.provider_type.parse().unwrap(),
            model: comparing_prompt_run.model,
            temperature: comparing_prompt_run.temperature,
            max_tokens: comparing_prompt_run.max_token,
            response_format: None,
        })
    }

    async fn create_comparing_prompt_run(
        &self,
        param: ComparingPromptSettingRunModel,
    ) -> Result<i32, ApplicationError> {
        let comparing_prompt_run = comparing_prompt_runs::ActiveModel {
            id: Default::default(),
            manager_id: ActiveValue::Set(param.manager_id),
            provider_type: ActiveValue::Set(param.provider_type.to_string()),
            user_prompt: ActiveValue::Set(param.user_prompt),
            model: ActiveValue::Set(param.model),
            temperature: ActiveValue::Set(param.temperature),
            max_token: ActiveValue::Set(param.max_tokens),
            // response_format: ActiveValue::Set(param.response_format),
        };
        let inserted_comparing_prompt_run = ComparingPromptRuns::insert(comparing_prompt_run)
            .exec(self.db.as_ref())
            .await?;
        let comparing_prompt_run_id = inserted_comparing_prompt_run.last_insert_id;

        Ok(comparing_prompt_run_id)
    }
}

impl ComparingPromptRunRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ComparingPromptRunRepositoryImpl { db }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::thelper::db::setup_db;
    use crate::domain::comparing_prompt::ProviderType;
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
    async fn test_create_comparing_prompt_run() {
        let db = setup_db("test_create_comparing_prompt_run").await;
        let repository = ComparingPromptRunRepositoryImpl::new(Arc::clone(&db));

        // 事前データ
        let manager_id = seed_prompt_manager(Arc::clone(&db)).await;
        let _ = seed_comparing_prompt_manager(Arc::clone(&db), manager_id).await;

        // テスト対象のメソッドを呼び出し
        let result = repository
            .create_comparing_prompt_run(ComparingPromptSettingRunModel {
                id: 0,
                manager_id,
                user_prompt: "test_user_prompt".to_string(),
                provider_type: ProviderType::OpenAI,
                model: "test_model".to_string(),
                temperature: 0.0,
                max_tokens: None,
                response_format: None,
            })
            .await;

        // assert
        assert!(result.is_ok());
        let new_id = result.unwrap();
        let new_item = ComparingPromptRuns::find_by_id(new_id)
            .one(db.as_ref())
            .await
            .unwrap();
        assert!(new_item.is_some());
        let new_item = new_item.unwrap();
        assert_eq!(new_item.id, new_id);
        assert_eq!(new_item.manager_id, manager_id);
        assert_eq!(new_item.user_prompt, "test_user_prompt");
        assert_eq!(
            new_item.provider_type,
            ProviderType::OpenAI.to_string().as_str()
        );
        assert_eq!(new_item.model, "test_model");
        assert_eq!(new_item.temperature, 0.0);
        assert_eq!(new_item.max_token, None);
    }
}
