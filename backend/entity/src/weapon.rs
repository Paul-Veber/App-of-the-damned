use async_graphql::*;
use sea_orm::DeleteMany;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, SimpleObject, InputObject)]
#[graphql(input_name = "MultiLangInput")]
pub struct MultiLang {
    pub fr: Option<String>,
    pub eng: Option<String>
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "weapon")]
#[graphql(concrete(name = "Weapon", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: String,
    pub name: MultiLang,
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

