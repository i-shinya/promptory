use async_trait::async_trait;

use crate::common::errors::ApplicationError;

#[derive(Clone, Debug)]
pub struct ChatSettings {
    pub id: i32,
    pub user_prompt: String,
    pub system_prompt: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: Option<u16>,
    pub response_format: Option<String>,
}

// traitでasyncが使えない問題の対処
#[async_trait]
pub trait AIChat: Send + Sync {
    async fn do_chat(&self, settings: &ChatSettings) -> Result<String, ApplicationError>;
}
