use async_trait::async_trait;
use serde::Deserialize;

use crate::common::errors::ApplicationError;
use crate::domain::chat::{AIChat, ChatSettings};
use crate::domain::settings::SettingsRepository;

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
    async fn post_chat(&self, request: ChatRequest) -> Result<String, ApplicationError>;
}

#[derive(Clone, Debug)]
pub struct ChatUsecase<T, R>
where
    T: AIChat,
    R: SettingsRepository,
{
    ai_chat: T,
    settings_repository: R,
}

#[async_trait]
impl<T, R> Chat for ChatUsecase<T, R>
where
    T: AIChat,
    R: SettingsRepository,
{
    async fn post_chat(&self, request: ChatRequest) -> Result<String, ApplicationError> {
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

        let res = self
            .settings_repository
            .create_settings("title", "api_type")
            .await;
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
    R: SettingsRepository,
{
    pub fn new(chat: T, setting_repository: R) -> Self {
        ChatUsecase {
            ai_chat: chat,
            settings_repository: setting_repository,
        }
    }
}

// TODO 異常系のテストはそのうち追加する
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use tokio::sync::Mutex;

    use crate::common::errors::ApplicationError;
    use crate::domain::chat::ChatSettings;
    use crate::domain::settings::SettingsModel;

    use super::*;

    struct MockAIChat {
        expected_prompt: Arc<Mutex<String>>,
    }

    struct MockSettingsRepository {
        // TODO テスト用のフィールドを追加
    }

    #[async_trait]
    impl AIChat for MockAIChat {
        async fn do_chat(&self, settings: &ChatSettings) -> Result<String, ApplicationError> {
            let expected_prompt = self.expected_prompt.lock().await;
            assert_eq!(settings.user_prompt, *expected_prompt);
            Ok("Test response".to_string())
        }
    }

    #[async_trait]
    impl SettingsRepository for MockSettingsRepository {
        async fn find_settings(&self) -> Result<Vec<SettingsModel>, ApplicationError> {
            Ok(Vec::new())
        }

        async fn create_settings(
            &self,
            title: &str,
            api_type: &str,
        ) -> Result<i32, ApplicationError> {
            Ok(0)
        }
    }

    #[tokio::test]
    async fn test_post_chat() {
        let expected_prompt = "Test prompt".to_string();
        let mock_chat = MockAIChat {
            expected_prompt: Arc::new(Mutex::new(expected_prompt.clone())),
        };
        let mock_settings_repository = MockSettingsRepository {};
        let chat_usecase = ChatUsecase {
            ai_chat: mock_chat,
            settings_repository: mock_settings_repository,
        };
        let request = ChatRequest {
            user_prompt: expected_prompt,
            system_prompt: "".to_string(),
            model: "".to_string(),
            temperature: 0.0,
            response_format: None,
        };
        let result = chat_usecase.post_chat(request).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test response");
    }
}
