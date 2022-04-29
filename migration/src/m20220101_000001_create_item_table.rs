use entity::item;
use entity::maker;
use entity::user;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_item_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .create_table(
                sea_query::Table::create()
                    .table(item::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(item::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(item::Column::Title).string().not_null())
                    .col(ColumnDef::new(item::Column::Name).string().not_null())
                    .col(
                        ColumnDef::new(item::Column::ProductCode)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(item::Column::ArrivalDate).date())
                    .col(ColumnDef::new(item::Column::ReservationStartDate).date())
                    .col(ColumnDef::new(item::Column::ReservationDeadline).date())
                    .col(ColumnDef::new(item::Column::OrderDate).date())
                    .col(ColumnDef::new(item::Column::Sku).integer())
                    .col(ColumnDef::new(item::Column::IllustStatus).string())
                    .col(ColumnDef::new(item::Column::DesignStatus).string())
                    .col(ColumnDef::new(item::Column::LastUpdated).date().not_null())
                    .col(
                        ColumnDef::new(item::Column::RetailPrice)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(item::Column::CatalogStatus).string())
                    .col(ColumnDef::new(item::Column::AnnouncementStatus).string())
                    .col(ColumnDef::new(item::Column::Remarks).string())
                    // 外部キー
                    .col(ColumnDef::new(item::Column::MakerId).integer())
                    .col(ColumnDef::new(item::Column::PicId).integer())
                    .col(ColumnDef::new(item::Column::DoubleCheckPersonId).integer())
                    .to_owned(),
            )
            .await;
        let _ = manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .from(item::Entity, item::Column::MakerId)
                    .to(maker::Entity, maker::Column::Id)
                    .to_owned(),
            )
            .await;
        let _ = manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .from(item::Entity, item::Column::PicId)
                    .to(maker::Entity, user::Column::Id)
                    .to_owned(),
            )
            .await;
        let _ = manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .from(item::Entity, item::Column::DoubleCheckPersonId)
                    .to(maker::Entity, user::Column::Id)
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(item::Entity).to_owned())
            .await
    }
}
