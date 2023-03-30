pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230329_212951_weapon;
mod m20230330_221521_weapon_unit;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230329_212951_weapon::Migration),
            Box::new(m20230330_221521_weapon_unit::Migration),
        ]
    }
}
