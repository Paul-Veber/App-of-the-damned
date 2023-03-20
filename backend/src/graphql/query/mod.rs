
use entity::async_graphql;

pub mod unit;

pub use unit::UnitQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(UnitQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UnitQuery);
