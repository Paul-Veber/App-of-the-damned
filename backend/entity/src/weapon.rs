use async_graphql::{SimpleObject, InputObject};
use sea_orm::DeleteMany;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::unit::Characteristics;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, SimpleObject, InputObject)]
#[graphql(input_name = "MultiLangInput")]
pub struct MultiLang {
    pub fr: Option<String>,
    pub eng: Option<String>
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, SimpleObject, InputObject)]
#[graphql(input_name = "Special")]
pub struct Special {
    name: MultiLang,
    effect: MultiLang
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, SimpleObject)]
pub struct SpecialField {
    special: Vec<Special>
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "weapon")]
#[graphql(concrete(name = "Weapon", params()))]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub id: String,
    pub name: MultiLang,
    pub description: MultiLang,
    pub bonus: Option<Characteristics>,
    pub price: i32,
    pub special: Option<SpecialField>,
    pub wielding: Option<String>,
    pub shield: Option<bool>,
    pub range: Option<i32>,
    pub strength: Option<i32>
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: String) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn delete_by_id(id: String) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}

impl Related<super::unit::Entity> for Entity {
    fn to() -> RelationDef {
        super::weapon_unit::Relation::Unit.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::weapon_unit::Relation::Weapon.def().rev())
    }
}
