use async_trait::async_trait;

use crate::common::errors::ApplicationError;

#[derive(Clone, Debug)]
pub struct ComparingPromptSettingModel {
    pub id: i32,
    pub manager_id: i32,
    pub current_version: i32,
    pub versions: Vec<ComparingPromptSettingVersionModel>,
}

#[derive(Clone, Debug)]
pub struct ComparingPromptSettingVersionModel {
    pub id: i32,
    pub setting_id: i32,
    pub version: i32,
    pub system_prompt: String,
}

#[async_trait]
pub trait ComparingPromptSettingRepository: Send + Sync {
    async fn find_comparing_prompt_setting_by_id(
        &self,
        id: i32,
    ) -> Result<ComparingPromptSettingModel, ApplicationError>;

    async fn find_all_comparing_prompt_settings_by_manager_id(
        &self,
        manager_id: i32,
    ) -> Result<Vec<ComparingPromptSettingModel>, ApplicationError>;

    async fn create_comparing_prompt_setting(
        &self,
        manager_id: i32,
    ) -> Result<i32, ApplicationError>;
}
