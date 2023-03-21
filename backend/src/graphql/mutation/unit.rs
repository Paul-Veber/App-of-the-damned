use async_graphql::{Context, Object, Result, Json};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::unit;
use entity::sea_orm::{ActiveModelTrait, Set};
use serde::{Serialize, Deserialize};

use crate::db::Database;

#[derive(SimpleObject, Serialize, Deserialize)]
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

#[derive(InputObject)]
pub struct CreateUnitInput {
    pub name: String,
    pub unit_type: String,
    pub ballistic_weapon: String,
    pub weapons: Vec<String>,
    pub magic:Option<Vec<String>>, 
    pub skills: Option<Vec<String>>,
    pub experience: u32,
    pub characteristics: Json<Characteristics>,
    pub price: u32,
    pub description: String,

}

#[derive(SimpleObject)]
pub struct DeleteResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct UnitMutation;

#[Object]
impl UnitMutation {
    pub async fn create_unit(
        &self,
        ctx: &Context<'_>,
        input: CreateUnitInput,
    ) -> Result<unit::Model> {
        let db = ctx.data::<Database>().unwrap();

        let unit = unit::ActiveModel {
            name: Set(input.name),
            ..Default::default()
        };

        Ok(unit.insert(db.get_connection()).await?)
    }

    pub async fn delete_unit(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

        let res = unit::Entity::delete_by_id(id)
            .exec(db.get_connection())
            .await?;

        if res.rows_affected <= 1 {
            Ok(DeleteResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            unimplemented!()
        }
    }
}
