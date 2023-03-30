use sea_orm::EnumIter;
use sea_orm::entity::prelude::*;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SimpleObject, DeriveEntityModel)]
#[sea_orm(table_name = "weapon_unit")]
#[graphql(concrete(name = "WeaponUnit", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    unit_id: i32,
    #[sea_orm(primary_key)]
    weapon_id: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::unit::Entity",
        from = "Column::UnitId",
        to = "super::unit::Column::Id"
    )]
    Unit,
    #[sea_orm(
        belongs_to = "super::weapon::Entity",
        from = "Column::WeaponId",
        to = "super::weapon::Column::Id"
    )]
    Weapon,
}

impl ActiveModelBehavior for ActiveModel {}
