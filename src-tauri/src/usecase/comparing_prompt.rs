use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::common::errors::ApplicationError;
use crate::domain::chat::{AIChat, ChatSettings};
use crate::domain::comparing_prompt_setting::ComparingPromptSettingRepository;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddComparingPromptSettingRequest {
    pub manager_id: i32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddComparingPromptSettingResponse {
    pub id: i32,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetComparingPromptSettingRequest {
    pub id: i32,
}

type GetComparingPromptSettingResponse = ComparingPromptSettingItem;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetComparingPromptSettingsRequest {
    pub manager_id: i32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetComparingPromptSettingsResponse {
    pub settings: Vec<ComparingPromptSettingItem>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComparingPromptSettingItem {
    pub id: i32,
    pub manager_id: i32,
    pub version: i32,
    pub system_prompt: String,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // jsonデコードする際にキャメルケースをスネークケースに変換する
pub struct RunChatRequest {
    pub run_id: i32,
    pub user_prompt: String,
    pub system_prompt: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: Option<u16>,
    pub response_format: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunChatResponse {
    pub answer: String,
}

#[async_trait]
pub trait ComparingPrompt: Send + Sync {
    async fn add_comparing_prompt_setting(
        &self,
        request: AddComparingPromptSettingRequest,
    ) -> Result<AddComparingPromptSettingResponse, ApplicationError>;

    async fn get_comparing_prompt_setting(
        &self,
        request: GetComparingPromptSettingRequest,
    ) -> Result<GetComparingPromptSettingResponse, ApplicationError>;

    async fn get_all_comparing_prompt_settings(
        &self,
        request: GetComparingPromptSettingsRequest,
    ) -> Result<GetComparingPromptSettingsResponse, ApplicationError>;

    async fn run_chat(&self, request: RunChatRequest) -> Result<RunChatResponse, ApplicationError>;
}

#[derive(Clone, Debug)]
pub struct ChatUsecase<T, R>
where
    T: AIChat,
    R: ComparingPromptSettingRepository,
{
    ai_chat: Arc<T>,
    comparing_prompt_setting_repository: Arc<R>,
}

#[async_trait]
impl<T, R> ComparingPrompt for ChatUsecase<T, R>
where
    T: AIChat,
    R: ComparingPromptSettingRepository,
{
    async fn add_comparing_prompt_setting(
        &self,
        request: AddComparingPromptSettingRequest,
    ) -> Result<AddComparingPromptSettingResponse, ApplicationError> {
        let id = self
            .comparing_prompt_setting_repository
            .create_comparing_prompt_setting(request.manager_id)
            .await?;
        Ok(AddComparingPromptSettingResponse { id })
    }

    async fn get_comparing_prompt_setting(
        &self,
        request: GetComparingPromptSettingRequest,
    ) -> Result<GetComparingPromptSettingResponse, ApplicationError> {
        let setting = self
            .comparing_prompt_setting_repository
            .find_comparing_prompt_setting_by_id(request.id)
            .await?;
        Ok(ComparingPromptSettingItem {
            id: setting.id,
            manager_id: setting.manager_id,
            version: setting.current_version,
            system_prompt: "".to_string(),
        })
    }

    async fn get_all_comparing_prompt_settings(
        &self,
        request: GetComparingPromptSettingsRequest,
    ) -> Result<GetComparingPromptSettingsResponse, ApplicationError> {
        let settings = self
            .comparing_prompt_setting_repository
            .find_all_comparing_prompt_settings_by_manager_id(request.manager_id)
            .await?;
        let settings = settings
            .into_iter()
            .map(|setting| ComparingPromptSettingItem {
                id: setting.id,
                manager_id: setting.manager_id,
                version: setting.current_version,
                system_prompt: "".to_string(),
            })
            .collect();
        Ok(GetComparingPromptSettingsResponse { settings })
    }

    async fn run_chat(&self, request: RunChatRequest) -> Result<RunChatResponse, ApplicationError> {
        let settings = ChatSettings {
            id: 0,
            user_prompt: request.user_prompt.clone(),
            system_prompt: request.system_prompt.clone(),
            model: request.model.clone(),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            response_format: request.response_format.clone(),
        };
        let res = self.ai_chat.do_chat(&settings).await;
        // if let Err(err) = res {
        //     log::error!("post_chat error: {}", err);
        //     return Err(err);
        // }
        match res {
            Ok(response) => Ok(RunChatResponse { answer: response }),
            Err(err) => {
                log::error!("post_chat error: {}", err);
                Err(err)
            }
        }
    }
}

impl<T, R> ChatUsecase<T, R>
where
    T: AIChat,
    R: ComparingPromptSettingRepository,
{
    pub fn new(chat: Arc<T>, prompt_manager_repository: Arc<R>) -> Self {
        ChatUsecase {
            ai_chat: chat,
            comparing_prompt_setting_repository: prompt_manager_repository,
        }
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use sea_orm::DbErr;

    use crate::common::errors::ApplicationError;
    use crate::domain::chat::ChatSettings;
    use crate::domain::comparing_prompt_setting::ComparingPromptSettingModel;

    use super::*;

    // default mocking
    struct MockAIChat {}
    struct MockAIChatError {}
    struct MockComparingPromptSettingRepository {}
    struct MockComparingPromptSettingRepositoryError {}
    #[async_trait]
    impl AIChat for MockAIChat {
        async fn do_chat(&self, _settings: &ChatSettings) -> Result<String, ApplicationError> {
            Ok("Test response".to_string())
        }
    }

    #[async_trait]
    impl ComparingPromptSettingRepository for MockComparingPromptSettingRepository {
        async fn find_comparing_prompt_setting_by_id(
            &self,
            _id: i32,
        ) -> Result<ComparingPromptSettingModel, ApplicationError> {
            Ok(ComparingPromptSettingModel {
                id: 1,
                manager_id: 1,
                current_version: 1,
                versions: vec![],
            })
        }

        async fn find_all_comparing_prompt_settings_by_manager_id(
            &self,
            _manager_id: i32,
        ) -> Result<Vec<ComparingPromptSettingModel>, ApplicationError> {
            Ok(Vec::new())
        }

        async fn create_comparing_prompt_setting(
            &self,
            manager_id: i32,
        ) -> Result<i32, ApplicationError> {
            Ok(1)
        }
    }

    #[async_trait]
    impl AIChat for MockAIChatError {
        async fn do_chat(&self, _settings: &ChatSettings) -> Result<String, ApplicationError> {
            Err(ApplicationError::OpenAPIError("open ai error".to_string()))
        }
    }

    #[async_trait]
    impl ComparingPromptSettingRepository for MockComparingPromptSettingRepositoryError {
        async fn find_comparing_prompt_setting_by_id(
            &self,
            _id: i32,
        ) -> Result<ComparingPromptSettingModel, ApplicationError> {
            Err(ApplicationError::DBError(DbErr::Type(
                "db error".to_string(),
            )))
        }

        async fn find_all_comparing_prompt_settings_by_manager_id(
            &self,
            _manager_id: i32,
        ) -> Result<Vec<ComparingPromptSettingModel>, ApplicationError> {
            Err(ApplicationError::DBError(DbErr::Type(
                "db error".to_string(),
            )))
        }

        async fn create_comparing_prompt_setting(
            &self,
            _manager_id: i32,
        ) -> Result<i32, ApplicationError> {
            Err(ApplicationError::DBError(DbErr::Type(
                "db error".to_string(),
            )))
        }
    }

    #[tokio::test]
    async fn test_add_comparing_prompt_setting() {
        let mock_chat = MockAIChat {};
        let mock_comparing_prompt_setting_repository = MockComparingPromptSettingRepository {};
        let chat_usecase = ChatUsecase {
            ai_chat: Arc::new(mock_chat),
            comparing_prompt_setting_repository: Arc::new(mock_comparing_prompt_setting_repository),
        };
        let request = AddComparingPromptSettingRequest { manager_id: 1 };
        let result = chat_usecase.add_comparing_prompt_setting(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_comparing_prompt_setting_error() {
        let mock_chat = MockAIChat {};
        let mock_comparing_prompt_setting_repository = MockComparingPromptSettingRepositoryError {};
        let chat_usecase = ChatUsecase {
            ai_chat: Arc::new(mock_chat),
            comparing_prompt_setting_repository: Arc::new(mock_comparing_prompt_setting_repository),
        };
        let request = AddComparingPromptSettingRequest { manager_id: 1 };
        let result = chat_usecase.add_comparing_prompt_setting(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_comparing_prompt_setting() {
        let mock_chat = MockAIChat {};
        let mock_comparing_prompt_setting_repository = MockComparingPromptSettingRepository {};
        let chat_usecase = ChatUsecase {
            ai_chat: Arc::new(mock_chat),
            comparing_prompt_setting_repository: Arc::new(mock_comparing_prompt_setting_repository),
        };
        let request = GetComparingPromptSettingRequest { id: 1 };
        let result = chat_usecase.get_comparing_prompt_setting(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_comparing_prompt_setting_error() {
        let mock_chat = MockAIChat {};
        let mock_comparing_prompt_setting_repository = MockComparingPromptSettingRepositoryError {};
        let chat_usecase = ChatUsecase {
            ai_chat: Arc::new(mock_chat),
            comparing_prompt_setting_repository: Arc::new(mock_comparing_prompt_setting_repository),
        };
        let request = GetComparingPromptSettingRequest { id: 1 };
        let result = chat_usecase.get_comparing_prompt_setting(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_all_comparing_prompt_settings() {
        let mock_chat = MockAIChat {};
        let mock_comparing_prompt_setting_repository = MockComparingPromptSettingRepository {};
        let chat_usecase = ChatUsecase {
            ai_chat: Arc::new(mock_chat),
            comparing_prompt_setting_repository: Arc::new(mock_comparing_prompt_setting_repository),
        };
        let request = GetComparingPromptSettingsRequest { manager_id: 1 };
        let result = chat_usecase
            .get_all_comparing_prompt_settings(request)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_all_comparing_prompt_settings_error() {
        let mock_chat = MockAIChat {};
        let mock_comparing_prompt_setting_repository = MockComparingPromptSettingRepositoryError {};
        let chat_usecase = ChatUsecase {
            ai_chat: Arc::new(mock_chat),
            comparing_prompt_setting_repository: Arc::new(mock_comparing_prompt_setting_repository),
        };
        let request = GetComparingPromptSettingsRequest { manager_id: 1 };
        let result = chat_usecase
            .get_all_comparing_prompt_settings(request)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_run_chat() {
        let expected_prompt = "Test prompt".to_string();
        let mock_chat = MockAIChat {};
        let mock_comparing_prompt_setting_repository = MockComparingPromptSettingRepository {};
        let chat_usecase = ChatUsecase {
            ai_chat: Arc::new(mock_chat),
            comparing_prompt_setting_repository: Arc::new(mock_comparing_prompt_setting_repository),
        };
        let request = RunChatRequest {
            run_id: 1,
            user_prompt: expected_prompt,
            system_prompt: "".to_string(),
            model: "".to_string(),
            temperature: 0.0,
            max_tokens: None,
            response_format: None,
        };
        let result = chat_usecase.run_chat(request).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().answer, "Test response");
    }

    #[tokio::test]
    async fn test_run_chat_error() {
        struct MockAIChatError {}
        #[async_trait]
        impl AIChat for MockAIChatError {
            async fn do_chat(&self, _settings: &ChatSettings) -> Result<String, ApplicationError> {
                Err(ApplicationError::OpenAPIError("open ai error".to_string()))
            }
        }

        let expected_prompt = "Test prompt".to_string();
        let mock_chat = MockAIChatError {};
        let mock_comparing_prompt_setting_repository = MockComparingPromptSettingRepository {};
        let chat_usecase = ChatUsecase {
            ai_chat: Arc::new(mock_chat),
            comparing_prompt_setting_repository: Arc::new(mock_comparing_prompt_setting_repository),
        };
        let request = RunChatRequest {
            run_id: 1,
            user_prompt: expected_prompt,
            system_prompt: "".to_string(),
            model: "".to_string(),
            temperature: 0.0,
            max_tokens: None,
            response_format: None,
        };
        let result = chat_usecase.run_chat(request).await;
        assert!(result.is_err());
    }
}
