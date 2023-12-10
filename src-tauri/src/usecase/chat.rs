use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;

use crate::common::errors::ApplicationError;
use crate::domain::chat::{AIChat, ChatSettings};
use crate::domain::prompt_manager::PromptManagerRepository;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // jsonデコードする際にキャメルケースをスネークケースに変換する
pub struct ChatRequest {
    pub user_prompt: String,
    pub system_prompt: String,
    pub model: String,
    pub temperature: f32,
    pub response_format: Option<String>,
}

#[async_trait]
pub trait Chat: Send + Sync {
    async fn run_chat(&self, request: ChatRequest) -> Result<String, ApplicationError>;
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
    async fn run_chat(&self, request: ChatRequest) -> Result<String, ApplicationError> {
        let settings = ChatSettings {
            id: 0,
            user_prompt: request.user_prompt.clone(),
            system_prompt: request.system_prompt.clone(),
            model: request.model.clone(),
            temperature: request.temperature,
            response_format: request.response_format.clone(),
        };
        let res = self.ai_chat.do_chat(&settings).await;
        if let Err(err) = res {
            log::error!("post_chat error: {}", err);
            return Err(err);
        }
        let answer = res.unwrap();

        // DBに永続化
        let res = self.prompt_manager_repository.create("title").await;
        match res {
            Ok(_) => Ok(answer),
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
    use crate::domain::prompt_manager::PromptManagerModel;

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
        async fn find_all(&self) -> Result<Vec<PromptManagerModel>, ApplicationError> {
            Ok(Vec::new())
        }

        async fn create(&self, _title: &str) -> Result<i32, ApplicationError> {
            Ok(1)
        }

        async fn logical_delete(&self, _id: i32) -> Result<i32, ApplicationError> {
            Ok(1)
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
        let request = ChatRequest {
            user_prompt: expected_prompt,
            system_prompt: "".to_string(),
            model: "".to_string(),
            temperature: 0.0,
            response_format: None,
        };
        let result = chat_usecase.run_chat(request).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test response");
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
        let request = ChatRequest {
            user_prompt: expected_prompt,
            system_prompt: "".to_string(),
            model: "".to_string(),
            temperature: 0.0,
            response_format: None,
        };
        let result = chat_usecase.run_chat(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_post_chat_error_create_settings() {
        struct MockSettingsRepositoryError {}
        #[async_trait]
        impl PromptManagerRepository for MockSettingsRepositoryError {
            async fn find_all(&self) -> Result<Vec<PromptManagerModel>, ApplicationError> {
                Err(ApplicationError::DBError(DbErr::Type(
                    "db error".to_string(),
                )))
            }

            async fn create(&self, _title: &str) -> Result<i32, ApplicationError> {
                Err(ApplicationError::DBError(DbErr::Type(
                    "db error".to_string(),
                )))
            }

            async fn logical_delete(&self, _id: i32) -> Result<i32, ApplicationError> {
                Err(ApplicationError::DBError(DbErr::Type(
                    "db error".to_string(),
                )))
            }
        }

        let expected_prompt = "Test prompt".to_string();
        let mock_chat = MockAIChat {};
        let mock_settings_repository = MockSettingsRepositoryError {};
        let chat_usecase = ChatUsecase {
            ai_chat: Arc::new(mock_chat),
            prompt_manager_repository: Arc::new(mock_settings_repository),
        };
        let request = ChatRequest {
            user_prompt: expected_prompt,
            system_prompt: "system prompt".to_string(),
            model: "test_model".to_string(),
            temperature: 0.0,
            response_format: None,
        };
        let result = chat_usecase.run_chat(request).await;
        assert!(result.is_err());
    }
}
