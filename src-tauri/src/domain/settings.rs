use async_trait::async_trait;

use crate::common::errors::ApplicationError;

#[derive(Clone, Debug)]
pub struct SettingsModel {
    pub id: i32,
    pub title: String,
    pub api_type: String,
}

#[async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn find_settings(&self) -> Result<Vec<SettingsModel>, ApplicationError>;
    async fn create_settings(&self, title: &str, api_type: &str) -> Result<i32, ApplicationError>;
}
