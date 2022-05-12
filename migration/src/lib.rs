pub use sea_schema::migration::prelude::*;

mod m20220425_120000_create_maker_table;
mod m20220425_130000_create_user_table;
mod m20220503_000001_create_item_table;
mod m20220512_233538_add_deleted_column_to_makers_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220425_120000_create_maker_table::Migration),
            Box::new(m20220425_130000_create_user_table::Migration),
            Box::new(m20220503_000001_create_item_table::Migration),
            Box::new(m20220512_233538_add_deleted_column_to_makers_table::Migration),
        ]
    }
}
