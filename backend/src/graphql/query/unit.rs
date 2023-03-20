use async_graphql::{Context, Object, Result};
use entity::{async_graphql, unit, sea_orm::EntityTrait};

use crate::db::Database;

#[derive(Default)]
pub struct UnitQuery;

#[Object]
impl UnitQuery {
    async fn get_unit(&self, ctx: &Context<'_>) -> Result<Vec<unit::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(unit::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_unit_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<unit::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(unit::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}
