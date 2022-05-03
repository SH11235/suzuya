use entity::item;
use entity::maker;
use entity::user;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220503_000001_create_item_table"
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
                    .col(ColumnDef::new(item::Column::ReleaseDate).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(item::Column::ReservationStartDate)
                            .timestamp_with_time_zone(),
                    )
                    .col(
                        ColumnDef::new(item::Column::ReservationDeadline)
                            .timestamp_with_time_zone(),
                    )
                    .col(ColumnDef::new(item::Column::OrderDate).timestamp_with_time_zone())
                    .col(ColumnDef::new(item::Column::Title).string().not_null())
                    .col(
                        ColumnDef::new(item::Column::ProjectType)
                            .string()
                            .default("デフォルト")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(item::Column::LastUpdated)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(item::Column::Name).string().not_null())
                    .col(ColumnDef::new(item::Column::ProductCode).string())
                    .col(ColumnDef::new(item::Column::Sku).integer())
                    .col(
                        ColumnDef::new(item::Column::IllustStatus)
                            .string()
                            .default("未着手")
                            .not_null(),
                    )
                    .col(ColumnDef::new(item::Column::PicIllustId).integer())
                    .col(
                        ColumnDef::new(item::Column::DesignStatus)
                            .string()
                            .default("未着手")
                            .not_null(),
                    )
                    .col(ColumnDef::new(item::Column::PicDesignId).integer())
                    .col(ColumnDef::new(item::Column::MakerId).integer())
                    .col(ColumnDef::new(item::Column::RetailPrice).integer())
                    .col(ColumnDef::new(item::Column::DoubleCheckPersonId).integer())
                    .col(
                        ColumnDef::new(item::Column::CatalogStatus)
                            .string()
                            .default("未着手")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(item::Column::AnnouncementStatus)
                            .string()
                            .default("未着手")
                            .not_null(),
                    )
                    .col(ColumnDef::new(item::Column::Remarks).string())
                    .to_owned(),
            )
            .await;
        let _foreign_key_maker_id = manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .from(item::Entity, item::Column::MakerId)
                    .to(maker::Entity, maker::Column::Id)
                    .to_owned(),
            )
            .await;
        let _foreign_key_pic_illust_id = manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .from(item::Entity, item::Column::PicIllustId)
                    .to(user::Entity, user::Column::Id)
                    .to_owned(),
            )
            .await;
        let _foreign_key_pic_design_id = manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .from(item::Entity, item::Column::PicDesignId)
                    .to(user::Entity, user::Column::Id)
                    .to_owned(),
            )
            .await;

        let _foreign_key_double_check_person_id = manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .from(item::Entity, item::Column::DoubleCheckPersonId)
                    .to(user::Entity, user::Column::Id)
                    .to_owned(),
            )
            .await;

        let _create_index = manager
            .create_index(
                sea_query::Index::create()
                    .name("idx-item-title")
                    .table(item::Entity)
                    .col(item::Column::Title)
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
