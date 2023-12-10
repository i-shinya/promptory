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
    async fn find_all(&self) -> Result<Vec<PromptManagerModel>, ApplicationError>;
    async fn create(&self, title: &str) -> Result<i32, ApplicationError>;
    async fn logical_delete(&self, id: i32) -> Result<i32, ApplicationError>;
}
