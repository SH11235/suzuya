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
                    .col(ColumnDef::new(Title::OrderDateToMaker).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Title::ProjectType)
                            .string()
                            .default("デフォルト")
                            .not_null(),
                    )
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
    OrderDateToMaker,
    ProjectType,
    Deleted,
}

#[cfg(test)]
mod tests {
    use super::Title;
    use sea_orm_migration::prelude::*;

    #[test]
    fn test() {
        // test query string
        let table = Table::create()
            .table(Title::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Title::Id).uuid().not_null().primary_key(), // .default(Uuid::new_v4()), // comment out for testing
            )
            .col(ColumnDef::new(Title::Name).string().not_null())
            .col(ColumnDef::new(Title::ReleaseDate).timestamp_with_time_zone())
            .col(ColumnDef::new(Title::ReservationStartDate).timestamp_with_time_zone())
            .col(ColumnDef::new(Title::ReservationDeadline).timestamp_with_time_zone())
            .col(ColumnDef::new(Title::OrderDateToMaker).timestamp_with_time_zone())
            .col(
                ColumnDef::new(Title::ProjectType)
                    .string()
                    .default("デフォルト")
                    .not_null(),
            )
            .col(
                ColumnDef::new(Title::Deleted)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .to_owned();
        assert_eq!(
            table.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "title" ("#,
                r#""id" uuid NOT NULL PRIMARY KEY,"#,
                r#""name" varchar NOT NULL,"#,
                r#""release_date" timestamp with time zone,"#,
                r#""reservation_start_date" timestamp with time zone,"#,
                r#""reservation_deadline" timestamp with time zone,"#,
                r#""order_date_to_maker" timestamp with time zone,"#,
                r#""project_type" varchar DEFAULT 'デフォルト' NOT NULL,"#,
                r#""deleted" bool NOT NULL DEFAULT FALSE"#,
                r#")"#,
            ]
            .join(" ")
        );
    }
}
