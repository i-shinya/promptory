use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait,
    ModelTrait, QueryFilter, TransactionTrait,
};

use crate::common::errors::ApplicationError;
use crate::domain::prompt_manager::{
    APIType, ActionType, PromptManagerModel, PromptManagerRepository,
};
use crate::infra::repository::entities::prelude::{PromptManager, PromptManagerTag, Tag};
use crate::infra::repository::entities::{
    comparing_prompt_manager, prompt_manager, prompt_manager_tag, tag,
};

#[derive(Clone, Debug)]
pub struct PromptManagerRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

#[async_trait]
impl PromptManagerRepository for PromptManagerRepositoryImpl {
    async fn find_prompt_manager_by_id(
        &self,
        id: i32,
    ) -> Result<PromptManagerModel, ApplicationError> {
        let prompt_manager: Option<prompt_manager::Model> = PromptManager::find()
            .filter(prompt_manager::Column::Id.eq(id))
            .one(self.db.as_ref())
            .await?;

        if prompt_manager.is_none() {
            return Err(ApplicationError::EmptyResult);
        }

        let prompt_manager = prompt_manager.unwrap();
        let tags = prompt_manager
            .find_related(Tag)
            .all(self.db.as_ref())
            .await?;

        Ok(PromptManagerModel {
            id: prompt_manager.id,
            title: prompt_manager.title,
            action_type: prompt_manager.action_type.map(|a| a.parse().unwrap()),
            api_type: prompt_manager.api_type.map(|a| a.parse().unwrap()),
            tags: tags.into_iter().map(|t| t.value).collect(),
        })
    }

    async fn find_all_prompt_managers(&self) -> Result<Vec<PromptManagerModel>, ApplicationError> {
        let prompt_managers: Vec<(prompt_manager::Model, Vec<tag::Model>)> = PromptManager::find()
            .filter(prompt_manager::Column::DeletedAt.is_null())
            .find_with_related(Tag)
            .all(self.db.as_ref())
            .await?;

        Ok({
            prompt_managers
                .into_iter()
                .map(|s| PromptManagerModel {
                    id: s.0.id,
                    title: s.0.title,
                    action_type: s.0.action_type.map(|a| a.parse().unwrap()),
                    api_type: s.0.api_type.map(|a| a.parse().unwrap()),
                    tags: s.1.into_iter().map(|t| t.value).collect(),
                })
                .collect()
        })
    }

    async fn create_prompt_manager(&self, title: &str) -> Result<i32, ApplicationError> {
        let prompt_manager = prompt_manager::ActiveModel {
            id: Default::default(),
            title: ActiveValue::Set(title.to_string()),
            action_type: ActiveValue::Set(None), // 初期値はNone
            api_type: ActiveValue::Set(None),    // 初期値はNone
            deleted_at: ActiveValue::Set(None),  // 初期値はNone
        };
        let res = PromptManager::insert(prompt_manager)
            .exec(self.db.as_ref())
            .await
            .map_err(ApplicationError::DBError)?;
        Ok(res.last_insert_id)
    }

    async fn update_prompt_manager(
        &self,
        id: i32,
        title: &str,
        action_type: Option<ActionType>,
        api_type: Option<APIType>,
        tags: Vec<String>,
    ) -> Result<(), ApplicationError> {
        let txn = self
            .db
            .as_ref()
            .begin()
            .await
            .map_err(ApplicationError::DBError)?;

        let prompt_manager = PromptManager::find_by_id(id)
            .one(&txn)
            .await
            .map_err(ApplicationError::DBError)?;
        if prompt_manager.is_none() {
            return Err(ApplicationError::EmptyResult);
        }
        let prompt_manager = prompt_manager.unwrap();

        // 既存のcomparing_XXX_managerを削除する
        if let Some(action_type) = prompt_manager.clone().action_type {
            let action_type = ActionType::from_str(&action_type).map_err(|_| {
                ApplicationError::ParseError(
                    "failed to convert string to enum ActionType".to_string(),
                )
            })?;
            match action_type {
                ActionType::ComparingPrompt => {
                    let _ = comparing_prompt_manager::Entity::delete_many()
                        .filter(comparing_prompt_manager::Column::ManagerId.eq(id))
                        .exec(&txn)
                        .await
                        .map_err(ApplicationError::DBError)?;
                }
                ActionType::ComparingModel => {
                    // TODO comparing_model_managerを削除する
                    // let _ = comparing_model_manager::Entity::delete_many()
                    //     .filter(comparing_model_manager::Column::ManagerId.eq(id))
                    //     .exec(&txn)
                    //     .await
                    //     .map_err(ApplicationError::DBError)?;
                }
            }
        }

        let mut prompt_manager: prompt_manager::ActiveModel = prompt_manager.into();
        prompt_manager.title = ActiveValue::Set(title.to_string());
        prompt_manager.action_type = ActiveValue::Set(action_type.clone().map(|a| a.to_string()));
        prompt_manager.api_type = ActiveValue::Set(api_type.map(|a| a.to_string()));
        let _ = prompt_manager
            .update(&txn)
            .await
            .map_err(ApplicationError::DBError)?;

        // comparing_XXX_managerを作成する
        if let Some(action_type) = action_type {
            match action_type {
                ActionType::ComparingPrompt => {
                    let comparing_prompt_manager = comparing_prompt_manager::ActiveModel {
                        manager_id: ActiveValue::Set(id),
                    };
                    let _ = comparing_prompt_manager
                        .insert(&txn)
                        .await
                        .map_err(ApplicationError::DBError)?;
                }
                ActionType::ComparingModel => {
                    // TODO comparing_model_managerを作成する
                    // let comparing_model_manager = comparing_model_manager::ActiveModel {
                    //     manager_id: ActiveValue::Set(id),
                    // };
                    // let _ = comparing_model_manager
                    //     .insert(&txn)
                    //     .await
                    //     .map_err(ApplicationError::DBError)?;
                }
            }
        }

        // prompt_manager_tagを更新する
        // 既存のtagに一致するものがあれば取得、なければtagを作成する
        let mut tag_ids: Vec<i32> = Vec::new();
        for tag_str in tags {
            let tag = Tag::find()
                .filter(tag::Column::Value.eq(tag_str.clone()))
                .one(&txn)
                .await
                .map_err(ApplicationError::DBError)?;
            if tag.is_none() {
                let tag = tag::ActiveModel {
                    id: Default::default(),
                    value: ActiveValue::Set(tag_str),
                };
                let res = Tag::insert(tag)
                    .exec(&txn)
                    .await
                    .map_err(ApplicationError::DBError)?;
                tag_ids.push(res.last_insert_id);
            } else {
                tag_ids.push(tag.unwrap().id);
            }
        }

        // 既存のprompt_manager_tagを削除
        let _ = prompt_manager_tag::Entity::delete_many()
            .filter(prompt_manager_tag::Column::PromptManagerId.eq(id))
            .exec(&txn)
            .await
            .map_err(ApplicationError::DBError)?;

        // prompt_manager_tagを作成
        for tag_id in tag_ids {
            let prompt_manager_tag = prompt_manager_tag::ActiveModel {
                id: Default::default(),
                prompt_manager_id: ActiveValue::Set(id),
                tag_id: ActiveValue::Set(tag_id),
            };
            let _ = PromptManagerTag::insert(prompt_manager_tag)
                .exec(&txn)
                .await
                .map_err(ApplicationError::DBError)?;
        }

        txn.commit().await.map_err(ApplicationError::DBError)?;
        Ok(())
    }

    async fn logical_delete_prompt_manager(&self, id: i32) -> Result<(), ApplicationError> {
        let prompt_manager = PromptManager::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(ApplicationError::DBError)?;
        if prompt_manager.is_none() {
            return Err(ApplicationError::EmptyResult);
        }
        let mut prompt_manager: prompt_manager::ActiveModel = prompt_manager.unwrap().into();
        prompt_manager.deleted_at = ActiveValue::Set(Some(chrono::Utc::now().to_string()));
        let _ = prompt_manager
            .update(self.db.as_ref())
            .await
            .map_err(ApplicationError::DBError)?;
        Ok(())
    }
}

impl PromptManagerRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        PromptManagerRepositoryImpl { db }
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::{ActiveValue, EntityTrait};

    use crate::common::thelper::db::setup_db;
    use crate::domain::prompt_manager::{APIType, ActionType, PromptManagerRepository};
    use crate::infra::repository::entities::prelude::{
        ComparingPromptManager, PromptManager, PromptManagerTag, Tag,
    };
    use crate::infra::repository::entities::{
        comparing_prompt_manager, prompt_manager, prompt_manager_tag, tag,
    };
    use crate::infra::repository::prompt_manager::PromptManagerRepositoryImpl;

    #[tokio::test]
    async fn test_find_prompt_manager_by_id() {
        let db = setup_db("test_find_prompt_manager_by_id").await;
        let repo = PromptManagerRepositoryImpl::new(db.clone());

        let prompt_manager = prompt_manager::ActiveModel {
            id: Default::default(),
            title: ActiveValue::Set("test_title".to_string()),
            action_type: ActiveValue::Set(Option::from(ActionType::ComparingPrompt.to_string())),
            api_type: ActiveValue::Set(None),
            deleted_at: ActiveValue::Set(None),
        };
        let inserted_prompt_manager = PromptManager::insert(prompt_manager)
            .exec(db.as_ref())
            .await
            .expect("Failed to insert prompt manager");
        let prompt_manager_id = inserted_prompt_manager.last_insert_id;

        let tag = tag::ActiveModel {
            id: Default::default(),
            value: ActiveValue::Set("test_tag".to_string()),
        };
        let inserted_tag = Tag::insert(tag)
            .exec(db.as_ref())
            .await
            .expect("Failed to insert tag");
        let tag_id = inserted_tag.last_insert_id;
        let prompt_manager_tag = prompt_manager_tag::ActiveModel {
            id: Default::default(),
            prompt_manager_id: ActiveValue::Set(prompt_manager_id),
            tag_id: ActiveValue::Set(tag_id),
        };
        PromptManagerTag::insert(prompt_manager_tag)
            .exec(db.as_ref())
            .await
            .expect("Failed to insert into prompt_manager_tag");

        let result = repo.find_prompt_manager_by_id(prompt_manager_id).await;
        assert!(result.is_ok());
        let manager = result.unwrap();

        // assert
        assert_eq!(manager.title, "test_title");
        assert_eq!(manager.action_type, Some(ActionType::ComparingPrompt));
        assert_eq!(manager.api_type, None);
        assert_eq!(manager.tags[0], "test_tag");
    }

    #[tokio::test]
    async fn test_find_prompt_manager_by_id_not_found_error() {
        let db = setup_db("test_find_prompt_manager_by_id_not_found_error").await;
        let repo = PromptManagerRepositoryImpl::new(db.clone());

        // 存在しないIDでfindメソッドを呼び出し
        let result = repo.find_prompt_manager_by_id(9999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_all_prompt_managers() {
        let db = setup_db("test_find_all_prompt_managers").await;
        let repo = PromptManagerRepositoryImpl::new(db.clone());

        let prompt_manager = prompt_manager::ActiveModel {
            id: Default::default(),
            title: ActiveValue::Set("test_title".to_string()),
            action_type: ActiveValue::Set(Option::from(ActionType::ComparingPrompt.to_string())),
            api_type: ActiveValue::Set(None),
            deleted_at: ActiveValue::Set(None),
        };
        let inserted_prompt_manager = PromptManager::insert(prompt_manager)
            .exec(db.as_ref())
            .await
            .expect("Failed to insert prompt manager");
        let prompt_manager_id = inserted_prompt_manager.last_insert_id;

        let tag = tag::ActiveModel {
            id: Default::default(),
            value: ActiveValue::Set("test_tag".to_string()),
        };
        let inserted_tag = Tag::insert(tag)
            .exec(db.as_ref())
            .await
            .expect("Failed to insert tag");
        let tag_id = inserted_tag.last_insert_id;
        let prompt_manager_tag = prompt_manager_tag::ActiveModel {
            id: Default::default(),
            prompt_manager_id: ActiveValue::Set(prompt_manager_id),
            tag_id: ActiveValue::Set(tag_id),
        };
        PromptManagerTag::insert(prompt_manager_tag)
            .exec(db.as_ref())
            .await
            .expect("Failed to insert into prompt_manager_tag");

        let result = repo.find_all_prompt_managers().await;
        assert!(result.is_ok());
        let managers = result.unwrap();

        // assert
        assert_eq!(managers.len(), 1);
        assert_eq!(managers[0].title, "test_title");
        assert_eq!(managers[0].action_type, Some(ActionType::ComparingPrompt));
        assert_eq!(managers[0].api_type, None);
        assert_eq!(managers[0].tags[0], "test_tag");
    }

    #[tokio::test]
    async fn test_create_prompt_managers() {
        let db = setup_db("test_create_prompt_managers").await;
        let repo = PromptManagerRepositoryImpl::new(db.clone());

        // create_prompt_managerメソッドを呼び出し
        let result = repo.create_prompt_manager("test_title").await;
        assert!(result.is_ok());
        let id = result.unwrap();

        // assert
        let prompt_managers = PromptManager::find_by_id(id)
            .one(db.as_ref())
            .await
            .expect("Failed to insert prompt manager");
        let settings = prompt_managers.unwrap();
        assert_eq!(settings.title, "test_title");
        assert_eq!(settings.action_type, None);
    }

    #[tokio::test]
    async fn test_update_prompt_manager() {
        let db = setup_db("test_update_prompt_manager").await;
        let repo = PromptManagerRepositoryImpl::new(db.clone());

        // 事前データ
        let prompt_manager = prompt_manager::ActiveModel {
            id: Default::default(),
            title: ActiveValue::Set("test_title".to_string()),
            action_type: ActiveValue::Set(Some(ActionType::ComparingPrompt.to_string())),
            api_type: ActiveValue::Set(None),
            deleted_at: ActiveValue::Set(None),
        };
        let inserted_prompt_manager = PromptManager::insert(prompt_manager)
            .exec(db.as_ref())
            .await
            .expect("Failed to insert prompt manager");
        let prompt_manager_id = inserted_prompt_manager.last_insert_id;

        let comparing_prompt_manager = comparing_prompt_manager::ActiveModel {
            manager_id: ActiveValue::Set(prompt_manager_id),
        };
        let _ = ComparingPromptManager::insert(comparing_prompt_manager)
            .exec(db.as_ref())
            .await
            .expect("Failed to insert comparing_prompt_manager");

        // update_prompt_managerメソッドを呼び出し
        let result = repo
            .update_prompt_manager(
                prompt_manager_id,
                "test_title2",
                Some(ActionType::ComparingPrompt),
                Some(APIType::Chat),
                vec!["test_tag".to_string()],
            )
            .await;
        assert!(result.is_ok());

        // prompt_managerのassert
        let prompt_manager = PromptManager::find_by_id(prompt_manager_id)
            .one(db.as_ref())
            .await
            .expect("Failed to insert prompt manager");
        let prompt_manager = prompt_manager.unwrap();
        assert_eq!(prompt_manager.title, "test_title2");
        assert_eq!(
            prompt_manager.action_type,
            Some(ActionType::ComparingPrompt.to_string())
        );
        assert_eq!(prompt_manager.api_type, Some(APIType::Chat.to_string()));

        // comparing_prompt_managerのassert
        let comparing_prompt_manager = ComparingPromptManager::find_by_id(prompt_manager_id)
            .one(db.as_ref())
            .await
            .expect("Failed to fetch comparing_prompt_manager");
        assert!(comparing_prompt_manager.is_some());

        // prompt_manager_tagsとtagsテーブルのassertを追加
        let prompt_manager_tags = PromptManagerTag::find()
            .all(db.as_ref())
            .await
            .expect("Failed to fetch prompt_manager_tags");
        assert_eq!(prompt_manager_tags.len(), 1);
        assert_eq!(prompt_manager_tags[0].prompt_manager_id, prompt_manager_id);
        assert_eq!(prompt_manager_tags[0].tag_id, 1);

        let tags = Tag::find_by_id(1)
            .one(db.as_ref())
            .await
            .expect("Failed to fetch tag");
        assert!(tags.is_some());
        assert_eq!(tags.unwrap().value, "test_tag");
    }

    #[tokio::test]
    async fn test_update_prompt_manager_not_found_error() {
        let db = setup_db("test_update_prompt_manager_not_found_error").await;
        let repo = PromptManagerRepositoryImpl::new(db.clone());

        // 存在しないIDでupdateメソッドを呼び出し
        let result = repo
            .update_prompt_manager(
                9999,
                "test_title2",
                Some(ActionType::ComparingModel),
                Some(APIType::Chat),
                vec!["test_tag".to_string()],
            )
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_logical_delete_prompt_managers() {
        let db = setup_db("test_logical_delete_prompt_managers").await;
        let repo = PromptManagerRepositoryImpl::new(db.clone());

        let prompt_manager = prompt_manager::ActiveModel {
            id: Default::default(),
            title: ActiveValue::Set("test_title".to_string()),
            action_type: ActiveValue::Set(Option::from(ActionType::ComparingPrompt.to_string())),
            api_type: ActiveValue::Set(None),
            deleted_at: ActiveValue::Set(None),
        };
        let _ = PromptManager::insert(prompt_manager)
            .exec(db.as_ref())
            .await
            .expect("Failed to insert prompt manager");

        // 作成した設定を削除
        let result = repo.logical_delete_prompt_manager(1).await;
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
    async fn test_logical_delete_prompt_managers_not_found_error() {
        let db = setup_db("test_logical_delete_prompt_managers_not_found_error").await;
        let repo = PromptManagerRepositoryImpl::new(db.clone());

        // 存在しないIDでlogical_deleteメソッドを呼び出し
        let result = repo.logical_delete_prompt_manager(9999).await;
        assert!(result.is_err());
    }
}
