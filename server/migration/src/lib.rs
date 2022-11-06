pub use sea_orm_migration::prelude::*;

mod m20221017_030456_maker;
mod m20221017_030528_title;
mod m20221017_223732_worker;
mod m20221017_224042_item;
mod m20221105_071832_add_resubmission_line_to_item;
mod m20221106_033801_add_updated_at_to_title;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221017_030456_maker::Migration),
            Box::new(m20221017_030528_title::Migration),
            Box::new(m20221017_223732_worker::Migration),
            Box::new(m20221017_224042_item::Migration),
            Box::new(m20221105_071832_add_resubmission_line_to_item::Migration),
            Box::new(m20221106_033801_add_updated_at_to_title::Migration),
        ]
    }
}
