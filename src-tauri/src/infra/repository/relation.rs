use sea_orm::{Related, RelationDef, RelationTrait};

use crate::infra::repository::entities;

impl Related<entities::tag::Entity> for entities::prompt_manager::Entity {
    fn to() -> RelationDef {
        entities::prompt_manager_tag::Relation::Tag.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            entities::prompt_manager_tag::Relation::PromptManager
                .def()
                .rev(),
        )
    }
}

impl Related<entities::prompt_manager::Entity> for entities::tag::Entity {
    fn to() -> RelationDef {
        entities::prompt_manager_tag::Relation::PromptManager.def()
    }

    fn via() -> Option<RelationDef> {
        Some(entities::prompt_manager_tag::Relation::Tag.def().rev())
    }
}
