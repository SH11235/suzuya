use sea_orm::prelude::Uuid;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Title::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Title::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Uuid::new_v4()),
                    )
                    .col(ColumnDef::new(Title::Name).string().not_null())
                    .col(ColumnDef::new(Title::ReleaseDate).timestamp_with_time_zone())
                    .col(ColumnDef::new(Title::ReservationStartDate).timestamp_with_time_zone())
                    .col(ColumnDef::new(Title::ReservationDeadline).timestamp_with_time_zone())
                    .col(ColumnDef::new(Title::OrderDate).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Title::Deleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Title::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Title {
    Table,
    Id,
    Name,
    ReleaseDate,
    ReservationStartDate,
    ReservationDeadline,
    OrderDate,
    Deleted,
}
