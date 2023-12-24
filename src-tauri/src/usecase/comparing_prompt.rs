use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::common::errors::ApplicationError;
use crate::domain::chat::{AIChat, ChatSettings};
use crate::domain::prompt_manager::PromptManagerRepository;

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
pub trait Chat: Send + Sync {
    async fn run_chat(&self, request: RunChatRequest) -> Result<RunChatResponse, ApplicationError>;
}

#[derive(Clone, Debug)]
pub struct ChatUsecase<T, R>
where
    T: AIChat,
    R: PromptManagerRepository,
{
    ai_chat: Arc<T>,
    prompt_manager_repository: Arc<R>,
}

#[async_trait]
impl<T, R> Chat for ChatUsecase<T, R>
where
    T: AIChat,
    R: PromptManagerRepository,
{
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
    R: PromptManagerRepository,
{
    pub fn new(chat: Arc<T>, prompt_manager_repository: Arc<R>) -> Self {
        ChatUsecase {
            ai_chat: chat,
            prompt_manager_repository,
        }
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use sea_orm::DbErr;

    use crate::common::errors::ApplicationError;
    use crate::domain::chat::ChatSettings;
    use crate::domain::prompt_manager::{APIType, ActionType, PromptManagerModel};

    use super::*;

    // default mocking
    struct MockAIChat {}
    struct MockSettingsRepository {}
    #[async_trait]
    impl AIChat for MockAIChat {
        async fn do_chat(&self, _settings: &ChatSettings) -> Result<String, ApplicationError> {
            Ok("Test response".to_string())
        }
    }

    #[async_trait]
    impl PromptManagerRepository for MockSettingsRepository {
        async fn find_prompt_manager_by_id(
            &self,
            _id: i32,
        ) -> Result<PromptManagerModel, ApplicationError> {
            Ok(PromptManagerModel {
                id: 1,
                title: "Test title".to_string(),
                action_type: None,
                api_type: None,
                tags: Vec::new(),
            })
        }

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
            _id: i32,
            _title: &str,
            _action_type: Option<ActionType>,
            _api_type: Option<APIType>,
            _tags: Vec<String>,
        ) -> Result<(), ApplicationError> {
            Ok(())
        }

        async fn logical_delete_prompt_manager(&self, _id: i32) -> Result<(), ApplicationError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_post_chat() {
        let expected_prompt = "Test prompt".to_string();
        let mock_chat = MockAIChat {};
        let mock_settings_repository = MockSettingsRepository {};
        let chat_usecase = ChatUsecase {
            ai_chat: Arc::new(mock_chat),
            prompt_manager_repository: Arc::new(mock_settings_repository),
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
    async fn test_post_chat_error_do_chat() {
        struct MockAIChatError {}
        #[async_trait]
        impl AIChat for MockAIChatError {
            async fn do_chat(&self, _settings: &ChatSettings) -> Result<String, ApplicationError> {
                Err(ApplicationError::OpenAPIError("open ai error".to_string()))
            }
        }

        let expected_prompt = "Test prompt".to_string();
        let mock_chat = MockAIChatError {};
        let mock_settings_repository = MockSettingsRepository {};
        let chat_usecase = ChatUsecase {
            ai_chat: Arc::new(mock_chat),
            prompt_manager_repository: Arc::new(mock_settings_repository),
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

    #[tokio::test]
    async fn test_post_chat_error_create_settings() {
        struct MockAIChatError {}
        #[async_trait]
        impl AIChat for MockAIChatError {
            async fn do_chat(&self, _settings: &ChatSettings) -> Result<String, ApplicationError> {
                Err(ApplicationError::OpenAPIError("open ai error".to_string()))
            }
        }

        struct MockSettingsRepositoryError {}
        #[async_trait]
        impl PromptManagerRepository for MockSettingsRepositoryError {
            async fn find_prompt_manager_by_id(
                &self,
                _id: i32,
            ) -> Result<PromptManagerModel, ApplicationError> {
                Err(ApplicationError::DBError(DbErr::Type(
                    "db error".to_string(),
                )))
            }

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
                _tags: Vec<String>,
            ) -> Result<(), ApplicationError> {
                Err(ApplicationError::DBError(DbErr::Type(
                    "db error".to_string(),
                )))
            }

            async fn logical_delete_prompt_manager(
                &self,
                _id: i32,
            ) -> Result<(), ApplicationError> {
                Err(ApplicationError::DBError(DbErr::Type(
                    "db error".to_string(),
                )))
            }
        }

        let expected_prompt = "Test prompt".to_string();
        let mock_chat = MockAIChatError {};
        let mock_settings_repository = MockSettingsRepositoryError {};
        let chat_usecase = ChatUsecase {
            ai_chat: Arc::new(mock_chat),
            prompt_manager_repository: Arc::new(mock_settings_repository),
        };
        let request = RunChatRequest {
            run_id: 1,
            user_prompt: expected_prompt,
            system_prompt: "system prompt".to_string(),
            model: "test_model".to_string(),
            temperature: 0.0,
            max_tokens: None,
            response_format: None,
        };
        let result = chat_usecase.run_chat(request).await;
        assert!(result.is_err());
    }
}
