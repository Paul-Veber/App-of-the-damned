use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::unit::{self, Characteristics};
use entity::sea_orm::{ActiveModelTrait, Set};
use sea_orm::ActiveValue::NotSet;
use entity::db::Database;

use crate::utils::mutation_value_check::mutation_value_check;

use super::characteristics::{update_characteristics, CharacteristicsUpdate};

#[derive(InputObject)]
pub struct CreateUnitInput {
    pub name: String,
    pub unit_type: String,
    pub rank: String,
    pub ballistic_weapon: String,
    // pub weapons: Vec<String>,
    pub magic:Option<Vec<String>>, 
    pub skills: Option<Vec<String>>,
    pub experience: i32,
    pub characteristics: Characteristics,
    pub price: i32,
    pub description: String,
}

#[derive(InputObject)]
pub struct UpdateUnitInput {
    pub name: Option<String>,
    pub unit_type: Option<String>,
    pub rank: Option<String>,
    pub ballistic_weapon: Option<String>,
    // pub weapons: Option<Vec<String>>,
    pub magic: Option<Option<Vec<String>>>, 
    pub skills: Option<Option<Vec<String>>>,
    pub experience: Option<i32>,
    pub characteristics: Option<CharacteristicsUpdate>,
    pub price: Option<i32>,
    pub description: Option<String>,
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
            unit_type: Set(input.unit_type),
            rank: Set(input.rank),
            ballistic_weapon: Set(input.ballistic_weapon),
            // weapons: Set(input.weapons),
            magic: Set(input.magic),
            skills: Set(input.skills),
            experience: Set(input.experience),
            characteristics: Set(input.characteristics),
            price: Set(input.price),
            description: Set(input.description),
            ..Default::default()
        };

        Ok(unit.insert(db.get_connection()).await?)
    }

    pub async fn update_unit(
        &self,
        ctx: &Context<'_>,
        id: i32,
        input: UpdateUnitInput
        ) -> Result<unit::Model> {
        
        let db = ctx.data::<Database>().unwrap();

        let unit: Option<_> = unit::Entity::find_by_id(id).one(db.get_connection()).await?;

        let mut unit: unit::ActiveModel = unit.unwrap().into();

        let updated_charac = match input.characteristics {
            Some(charac) => Set(update_characteristics(charac, unit.characteristics)),
            None => NotSet
        };

        unit.name = mutation_value_check(input.name); 
        unit.unit_type = mutation_value_check(input.unit_type);
        unit.rank = mutation_value_check(input.rank);
        unit.ballistic_weapon = mutation_value_check(input.ballistic_weapon);
        // unit.weapons = mutation_value_check(input.weapons);
        unit.magic = mutation_value_check(input.magic);
        unit.skills = mutation_value_check(input.skills);
        unit.experience = mutation_value_check(input.experience);
        unit.characteristics = updated_charac;
        unit.price = mutation_value_check(input.price);
        unit.description = mutation_value_check(input.description);
       
        Ok(unit.update(db.get_connection()).await?)
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
