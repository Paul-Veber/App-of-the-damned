use entity::async_graphql;

pub mod unit;

pub use unit::UnitMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(UnitMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(UnitMutation);
