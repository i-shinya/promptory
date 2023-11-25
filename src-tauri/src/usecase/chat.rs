use async_trait::async_trait;

use crate::common::errors::ApplicationError;
use crate::domain::chat::AIChat;

#[derive(Clone, Debug)]
pub struct ChatRequest {
    pub user_prompt: String,
    pub system_prompt: String,
}

#[async_trait]
pub trait Chat {
    async fn post_chat(&self, request: ChatRequest) -> Result<String, ApplicationError>;
}

#[derive(Clone, Debug)]
struct ChatUsecase<T>
where
    T: AIChat + ?Sized,
{
    chat_client: Box<T>,
}

pub fn new(chat: Box<dyn AIChat>) -> Box<dyn Chat> {
    Box::new(ChatUsecase { chat_client: chat })
}

#[async_trait]
impl<T> Chat for ChatUsecase<T>
where
    T: AIChat + ?Sized,
{
    async fn post_chat(&self, request: ChatRequest) -> Result<String, ApplicationError> {
        let settings = crate::domain::chat::ChatSettings {
            user_prompt: request.user_prompt.clone(),
            system_prompt: request.system_prompt.clone(),
        };
        match self.chat_client.do_chat(&settings).await {
            Ok(res) => Ok(res),
            Err(err) => {
                println!("post_chat error: {}", err);
                Err(err)
            }
        }
    }
}

// TODO 異常系のテストはそのうち追加する
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use tokio::sync::Mutex;

    use crate::domain::chat::ChatSettings;

    use super::*;

    struct MockAIChat {
        expected_prompt: Arc<Mutex<String>>,
    }

    #[async_trait]
    impl AIChat for MockAIChat {
        async fn do_chat(&self, settings: &ChatSettings) -> Result<String, ApplicationError> {
            let expected_prompt = self.expected_prompt.lock().await;
            assert_eq!(settings.user_prompt, *expected_prompt);
            Ok("Test response".to_string())
        }
    }

    #[tokio::test]
    async fn test_post_chat() {
        let expected_prompt = "Test prompt".to_string();
        let mock_chat = MockAIChat {
            expected_prompt: Arc::new(Mutex::new(expected_prompt.clone())),
        };
        let chat_usecase = ChatUsecase {
            chat_client: Box::new(mock_chat),
        };
        let request = ChatRequest {
            user_prompt: expected_prompt,
            system_prompt: "".to_string(),
        };
        let result = chat_usecase.post_chat(request).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test response");
    }
}
