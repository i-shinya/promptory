use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::common::errors::ApplicationError;

#[derive(Clone, Deserialize, Serialize, Debug, EnumString, Display, PartialEq)]
pub enum APIType {
    Chat,
    Assistant,
}

#[derive(Clone, Debug)]
pub struct PromptManagerModel {
    pub id: i32,
    pub title: String,
    pub api_type: Option<APIType>,
}

#[async_trait]
pub trait PromptManagerRepository: Send + Sync {
    async fn find_all_prompt_managers(&self) -> Result<Vec<PromptManagerModel>, ApplicationError>;
    async fn create_prompt_manager(&self, title: &str) -> Result<i32, ApplicationError>;
    async fn logical_delete_prompt_manager(&self, id: i32) -> Result<(), ApplicationError>;
}
