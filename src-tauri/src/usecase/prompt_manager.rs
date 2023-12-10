use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::common::errors::ApplicationError;
use crate::domain::prompt_manager::{APIType, PromptManagerRepository};

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // jsonデコードする際にキャメルケースをスネークケースに変換する
pub struct CreatePromptManagerRequest {
    pub title: String,
    pub api_type: Option<APIType>,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // jsonデコードする際にキャメルケースをスネークケースに変換する
pub struct CreatePromptManagerResponse {
    pub id: i32,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // jsonデコードする際にキャメルケースをスネークケースに変換する
pub struct UpdatePromptManagerRequest {
    pub id: i32,
    pub title: String,
    pub api_type: Option<APIType>,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // jsonデコードする際にキャメルケースをスネークケースに変換する
pub struct UpdatePromptManagerResponse {
    pub id: i32,
}

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
    pub api_type: Option<APIType>,
}

#[async_trait]
pub trait PromptManager: Send + Sync {
    async fn create_prompt_manager(
        &self,
        request: CreatePromptManagerRequest,
    ) -> Result<CreatePromptManagerResponse, ApplicationError>;

    async fn get_prompt_managers(
        &self,
        request: GetPromptManagerRequest,
    ) -> Result<GetPromptManagerResponse, ApplicationError>;
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

    async fn get_prompt_managers(
        &self,
        _request: GetPromptManagerRequest,
    ) -> Result<GetPromptManagerResponse, ApplicationError> {
        let res = self.prompt_manager_repository.find_prompt_manager().await;
        match res {
            Ok(mana) => {
                let managers = mana
                    .into_iter()
                    .map(|m| PromptManagerItem {
                        id: m.id,
                        title: m.title,
                        api_type: m.api_type,
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
    use crate::domain::prompt_manager::{PromptManagerModel, PromptManagerRepository};
    use crate::usecase::prompt_manager::{
        APIType, CreatePromptManagerRequest, GetPromptManagerRequest, PromptManager,
        PromptManagerUsecase,
    };

    struct MockPromptManagersRepository {}
    #[async_trait]
    impl PromptManagerRepository for MockPromptManagersRepository {
        async fn find_prompt_manager(&self) -> Result<Vec<PromptManagerModel>, ApplicationError> {
            Ok(Vec::new())
        }

        async fn create_prompt_manager(&self, _title: &str) -> Result<i32, ApplicationError> {
            Ok(1)
        }
    }

    struct MockPromptManagersRepositoryError {}
    #[async_trait]
    impl PromptManagerRepository for MockPromptManagersRepositoryError {
        async fn find_prompt_manager(&self) -> Result<Vec<PromptManagerModel>, ApplicationError> {
            Err(ApplicationError::DBError(DbErr::Type(
                "db error".to_string(),
            )))
        }

        async fn create_prompt_manager(&self, _title: &str) -> Result<i32, ApplicationError> {
            Err(ApplicationError::DBError(DbErr::Type(
                "db error".to_string(),
            )))
        }
    }

    #[tokio::test]
    async fn test_save_prompt_manager() {
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
    async fn test_save_prompt_manager_error() {
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
}
