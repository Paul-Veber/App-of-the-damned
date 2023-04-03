use async_graphql::*;
use sea_orm::{DeleteMany, FromJsonQueryResult};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use weapon::MultiLang;

use crate::db::Database;
use crate::weapon;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, SimpleObject, InputObject)]
#[graphql(input_name = "CharacteristicsInput")]
pub struct Characteristics {
   pub movement: u8,
   pub weapon_skill: u8,
   pub ballistic_skill: u8,
   pub strength: u8,
   pub toughness: u8,
   pub wounds: u8,
   pub initiative: u8,
   pub attacks: u8,
   pub leadership: u8,
   pub armor_save: u8,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "unit")]
#[graphql(name = "Unit", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub rank: String,
    pub unit_type: String,
    pub ballistic_weapon: String,
    // pub weapons: Vec<String>,
    pub magic: Option<Vec<String>>, 
    pub skills: Option<Vec<String>>,
    pub experience: i32,
    pub characteristics: Characteristics,
    pub price: i32,
    pub description: String,
}

#[ComplexObject]
impl Model {
  async fn weapon(&self, ctx: &Context<'_>) -> Result<Vec<weapon::Model>> {
      let db = ctx.data::<Database>().unwrap();

      Ok(self.find_related(weapon::Entity).all(db.get_connection()).await.map_err(|e| e.to_string())?)
  }
}


#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: i32) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}

impl Related<super::weapon::Entity> for Entity {
    fn to() -> RelationDef {
        super::weapon_unit::Relation::Weapon.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::weapon_unit::Relation::Unit.def().rev())
    }

}
