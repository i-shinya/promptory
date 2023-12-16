use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::common::errors::ApplicationError;

#[derive(Clone, Deserialize, Serialize, Debug, EnumString, Display, PartialEq)]
pub enum APIType {
    Chat,
    Assistant,
}

#[derive(Clone, Deserialize, Serialize, Debug, EnumString, Display, PartialEq)]
pub enum ActionType {
    ComparingPrompt,
    ComparingModel,
}

#[derive(Clone, Debug)]
pub struct PromptManagerModel {
    pub id: i32,
    pub title: String,
    pub action_type: Option<ActionType>,
    pub api_type: Option<APIType>,
    pub tags: Vec<String>,
}

#[async_trait]
pub trait PromptManagerRepository: Send + Sync {
    async fn find_prompt_manager_by_id(
        &self,
        id: i32,
    ) -> Result<PromptManagerModel, ApplicationError>;
    async fn find_all_prompt_managers(&self) -> Result<Vec<PromptManagerModel>, ApplicationError>;
    async fn create_prompt_manager(&self, title: &str) -> Result<i32, ApplicationError>;
    async fn logical_delete_prompt_manager(&self, id: i32) -> Result<(), ApplicationError>;
    async fn update_prompt_manager(
        &self,
        id: i32,
        title: &str,
        action_type: Option<ActionType>,
        api_type: Option<APIType>,
    ) -> Result<(), ApplicationError>;
}
