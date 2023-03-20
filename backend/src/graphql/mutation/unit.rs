use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::unit;
use entity::sea_orm::{ActiveModelTrait, Set};

use crate::db::Database;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreateUnitInput {
    pub name: String,
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
