use async_graphql::{EmptySubscription, Schema};
use entity::async_graphql;
use entity::db::Database;

use crate::{
    graphql::{mutation::Mutation, query::Query},
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema() -> AppSchema {
    let db = Database::new().await;

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish()
}
