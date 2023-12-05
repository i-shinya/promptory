use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

use crate::common::errors::ApplicationError;
use crate::common::errors::ApplicationError::DBError;
use crate::domain::settings::{SettingsModel, SettingsRepository};
use crate::infra::repository::entities::prelude::Settings;
use crate::infra::repository::entities::settings;

#[derive(Clone, Debug)]
pub struct SettingsRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

#[async_trait]
impl SettingsRepository for SettingsRepositoryImpl {
    async fn find_settings(&self) -> Result<Vec<SettingsModel>, ApplicationError> {
        let settings = Settings::find().all(self.db.as_ref()).await;
        match settings {
            Ok(setting) => {
                let mut result = Vec::new();
                for s in setting {
                    result.push(SettingsModel {
                        id: s.id,
                        title: s.title,
                        api_type: s.api_type,
                    });
                }
                Ok(result)
            }
            Err(err) => Err(DBError(err)),
        }
    }

    async fn create_settings(&self, title: &str, api_type: &str) -> Result<i32, ApplicationError> {
        let setting = settings::ActiveModel {
            id: Default::default(),
            title: ActiveValue::Set(title.to_string()),
            api_type: ActiveValue::Set(api_type.to_string()),
        };
        let res = Settings::insert(setting).exec(self.db.as_ref()).await?;
        Ok(res.last_insert_id)
    }
}

impl SettingsRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        SettingsRepositoryImpl { db }
    }
}
