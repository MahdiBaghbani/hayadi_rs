use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "person")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub telegram_id: Option<String>,
    pub name: Option<String>,
    pub gender: Option<i16>,
    pub birthday: Option<i16>,
    pub pref_min_age: Option<i16>,
    pub pref_max_age: Option<i16>,
    pub pref_genders: Option<i16>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::picture::Entity")]
    Picture
}

impl Related<super::picture::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Picture.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
