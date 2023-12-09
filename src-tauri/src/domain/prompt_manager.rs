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
    pub api_type: APIType,
}

#[async_trait]
pub trait PromptManagerRepository: Send + Sync {
    async fn find_settings(&self) -> Result<Vec<PromptManagerModel>, ApplicationError>;
    async fn create_settings(&self, title: &str, api_type: &str) -> Result<i32, ApplicationError>;
}
