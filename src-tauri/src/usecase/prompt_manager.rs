use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::common::errors::ApplicationError;
use crate::domain::prompt_manager::{APIType, ActionType, PromptManagerRepository};

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // jsonデコードする際にキャメルケースをスネークケースに変換する
pub struct CreatePromptManagerRequest {
    pub title: String,
    pub api_type: Option<APIType>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")] // jsonデコードする際にキャメルケースをスネークケースに変換する
pub struct CreatePromptManagerResponse {
    pub id: i32,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // jsonデコードする際にキャメルケースをスネークケースに変換する
pub struct UpdatePromptManagerRequest {
    pub id: i32,
    pub title: String,
    pub action_type: ActionType,
    pub api_type: Option<APIType>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")] // jsonデコードする際にキャメルケースをスネークケースに変換する
pub struct UpdatePromptManagerResponse {}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // jsonデコードする際にキャメルケースをスネークケースに変換する
pub struct GetPromptManagerRequest {}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")] // jsonデコードする際にキャメルケースをスネークケースに変換する
pub struct GetPromptManagerResponse {
    pub managers: Vec<PromptManagerItem>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PromptManagerItem {
    pub id: i32,
    pub title: String,
    pub action_type: Option<ActionType>,
    pub tags: Vec<String>,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // jsonデコードする際にキャメルケースをスネークケースに変換する
pub struct DeletePromptManagerRequest {
    pub id: i32,
}

#[async_trait]
pub trait PromptManager: Send + Sync {
    async fn get_prompt_managers(
        &self,
        request: GetPromptManagerRequest,
    ) -> Result<GetPromptManagerResponse, ApplicationError>;

    async fn create_prompt_manager(
        &self,
        request: CreatePromptManagerRequest,
    ) -> Result<CreatePromptManagerResponse, ApplicationError>;

    async fn logical_delete_prompt_managers(
        &self,
        request: DeletePromptManagerRequest,
    ) -> Result<(), ApplicationError>;

    async fn update_prompt_manager(
        &self,
        request: UpdatePromptManagerRequest,
    ) -> Result<UpdatePromptManagerResponse, ApplicationError>;
}

#[derive(Clone, Debug)]
pub struct PromptManagerUsecase<T>
where
    T: PromptManagerRepository,
{
    prompt_manager_repository: Arc<T>,
}

#[async_trait]
impl<T> PromptManager for PromptManagerUsecase<T>
where
    T: PromptManagerRepository,
{
    async fn get_prompt_managers(
        &self,
        _request: GetPromptManagerRequest,
    ) -> Result<GetPromptManagerResponse, ApplicationError> {
        let prompt_managers = self
            .prompt_manager_repository
            .find_all_prompt_managers()
            .await;

        match prompt_managers {
            Ok(mana) => {
                let managers = mana
                    .into_iter()
                    .map(|m| PromptManagerItem {
                        id: m.id,
                        title: m.title,
                        action_type: m.action_type,
                        tags: Vec::new(), // TODO 別PRにてDBからタグを取得するように修正する
                    })
                    .collect();
                Ok(GetPromptManagerResponse { managers })
            }
            Err(err) => {
                log::error!("get_prompt_managers error: {}", err);
                Err(err)
            }
        }
    }

    async fn create_prompt_manager(
        &self,
        request: CreatePromptManagerRequest,
    ) -> Result<CreatePromptManagerResponse, ApplicationError> {
        let res = self
            .prompt_manager_repository
            .create_prompt_manager(request.title.as_str())
            .await;

        match res {
            Ok(id) => Ok(CreatePromptManagerResponse { id }),
            Err(err) => {
                log::error!("create_prompt_manager error: {}", err);
                Err(err)
            }
        }
    }

    async fn update_prompt_manager(
        &self,
        request: UpdatePromptManagerRequest,
    ) -> Result<UpdatePromptManagerResponse, ApplicationError> {
        let res = self
            .prompt_manager_repository
            .update_prompt_manager(
                request.id,
                request.title.as_str(),
                Some(request.action_type),
                request.api_type,
            )
            .await;

        match res {
            Ok(id) => Ok(UpdatePromptManagerResponse {}),
            Err(err) => {
                log::error!("update_prompt_manager error: {}", err);
                Err(err)
            }
        }
    }

    async fn logical_delete_prompt_managers(
        &self,
        request: DeletePromptManagerRequest,
    ) -> Result<(), ApplicationError> {
        let res = self
            .prompt_manager_repository
            .logical_delete_prompt_manager(request.id)
            .await;
        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("logical_delete_prompt_managers error: {}", err);
                Err(err)
            }
        }
    }
}

impl<T> PromptManagerUsecase<T>
where
    T: PromptManagerRepository,
{
    pub fn new(prompt_manager_repository: Arc<T>) -> Self {
        PromptManagerUsecase {
            prompt_manager_repository,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use sea_orm::DbErr;

    use crate::common::errors::ApplicationError;
    use crate::domain::prompt_manager::{ActionType, PromptManagerModel, PromptManagerRepository};
    use crate::usecase::prompt_manager::{
        APIType, CreatePromptManagerRequest, DeletePromptManagerRequest, GetPromptManagerRequest,
        PromptManager, PromptManagerUsecase, UpdatePromptManagerRequest,
    };

    struct MockPromptManagersRepository {}
    #[async_trait]
    impl PromptManagerRepository for MockPromptManagersRepository {
        async fn find_all_prompt_managers(
            &self,
        ) -> Result<Vec<PromptManagerModel>, ApplicationError> {
            Ok(Vec::new())
        }

        async fn create_prompt_manager(&self, _title: &str) -> Result<i32, ApplicationError> {
            Ok(1)
        }

        async fn update_prompt_manager(
            &self,
            id: i32,
            title: &str,
            action_type: Option<ActionType>,
            api_type: Option<APIType>,
        ) -> Result<(), ApplicationError> {
            Ok(())
        }

        async fn logical_delete_prompt_manager(&self, _id: i32) -> Result<(), ApplicationError> {
            Ok(())
        }
    }

    struct MockPromptManagersRepositoryError {}
    #[async_trait]
    impl PromptManagerRepository for MockPromptManagersRepositoryError {
        async fn find_all_prompt_managers(
            &self,
        ) -> Result<Vec<PromptManagerModel>, ApplicationError> {
            Err(ApplicationError::DBError(DbErr::Type(
                "db error".to_string(),
            )))
        }

        async fn create_prompt_manager(&self, _title: &str) -> Result<i32, ApplicationError> {
            Err(ApplicationError::DBError(DbErr::Type(
                "db error".to_string(),
            )))
        }

        async fn update_prompt_manager(
            &self,
            _id: i32,
            _title: &str,
            _action_type: Option<ActionType>,
            _api_type: Option<APIType>,
        ) -> Result<(), ApplicationError> {
            Err(ApplicationError::DBError(DbErr::Type(
                "db error".to_string(),
            )))
        }

        async fn logical_delete_prompt_manager(&self, _id: i32) -> Result<(), ApplicationError> {
            Err(ApplicationError::DBError(DbErr::Type(
                "db error".to_string(),
            )))
        }
    }

    #[tokio::test]
    async fn test_get_prompt_managers() {
        let mock_repository = MockPromptManagersRepository {};
        let prompt_manager_usecase = PromptManagerUsecase {
            prompt_manager_repository: Arc::new(mock_repository),
        };
        let request = GetPromptManagerRequest {};
        let result = prompt_manager_usecase.get_prompt_managers(request).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().managers.len(), 0);
    }

    #[tokio::test]
    async fn test_get_prompt_managers_error() {
        let mock_repository = MockPromptManagersRepositoryError {};
        let prompt_manager_usecase = PromptManagerUsecase {
            prompt_manager_repository: Arc::new(mock_repository),
        };
        let request = GetPromptManagerRequest {};
        let result = prompt_manager_usecase.get_prompt_managers(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_prompt_manager() {
        let mock_repository = MockPromptManagersRepository {};
        let prompt_manager_usecase = PromptManagerUsecase {
            prompt_manager_repository: Arc::new(mock_repository),
        };
        let request = CreatePromptManagerRequest {
            title: "Test title".to_string(),
            api_type: Option::from(APIType::Chat),
        };
        let result = prompt_manager_usecase.create_prompt_manager(request).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, 1);
    }

    #[tokio::test]
    async fn test_create_prompt_manager_error() {
        let mock_repository = MockPromptManagersRepositoryError {};
        let prompt_manager_usecase = PromptManagerUsecase {
            prompt_manager_repository: Arc::new(mock_repository),
        };
        let request = CreatePromptManagerRequest {
            title: "Test title".to_string(),
            api_type: Option::from(APIType::Chat),
        };
        let result = prompt_manager_usecase.create_prompt_manager(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_prompt_manager() {
        let mock_repository = MockPromptManagersRepository {};
        let prompt_manager_usecase = PromptManagerUsecase {
            prompt_manager_repository: Arc::new(mock_repository),
        };
        let request = UpdatePromptManagerRequest {
            id: 1,
            title: "Test title".to_string(),
            action_type: ActionType::ComparingPrompt,
            api_type: Option::from(APIType::Chat),
        };
        let result = prompt_manager_usecase.update_prompt_manager(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_prompt_manager_error() {
        let mock_repository = MockPromptManagersRepositoryError {};
        let prompt_manager_usecase = PromptManagerUsecase {
            prompt_manager_repository: Arc::new(mock_repository),
        };
        let request = UpdatePromptManagerRequest {
            id: 1,
            title: "Test title".to_string(),
            action_type: ActionType::ComparingPrompt,
            api_type: Option::from(APIType::Chat),
        };
        let result = prompt_manager_usecase.update_prompt_manager(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_logical_delete_prompt_managers() {
        let mock_repository = MockPromptManagersRepository {};
        let prompt_manager_usecase = PromptManagerUsecase {
            prompt_manager_repository: Arc::new(mock_repository),
        };
        let request = DeletePromptManagerRequest { id: 1 };
        let result = prompt_manager_usecase
            .logical_delete_prompt_managers(request)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_logical_delete_prompt_managers_error() {
        let mock_repository = MockPromptManagersRepositoryError {};
        let prompt_manager_usecase = PromptManagerUsecase {
            prompt_manager_repository: Arc::new(mock_repository),
        };
        let request = DeletePromptManagerRequest { id: 1 };
        let result = prompt_manager_usecase
            .logical_delete_prompt_managers(request)
            .await;
        assert!(result.is_err());
    }
}
