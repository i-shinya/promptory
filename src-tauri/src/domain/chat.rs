use async_trait::async_trait;

use crate::common::errors::ApplicationError;

#[derive(Clone, Debug)]
pub struct ChatSettings {
    pub user_prompt: String,
    pub system_prompt: String,
}

// traitでasyncが使えない問題の対処
#[async_trait]
pub trait AIChat: Sync {
    async fn do_chat(&self, settings: &ChatSettings) -> Result<String, ApplicationError>;
}
