use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "picture")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub person_id: Uuid,
    pub r#type: i16,
    #[sea_orm(column_type = "Text")]
    pub data: String,
    pub format: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub vector: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
    belongs_to = "super::person::Entity",
    from = "Column::PersonId",
    to = "super::person::Column::Id"
    )]
    Person
}

impl Related<super::person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Person.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
