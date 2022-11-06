use crate::m20221017_030528_title::Title;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Title::Table)
                    .add_column(
                        ColumnDef::new(AddTitleColumn::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(sea_query::Value::ChronoDateTimeWithTimeZone(Some(
                                Box::new(chrono::Utc::now().into()),
                            ))),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Title::Table)
                    .drop_column(AddTitleColumn::UpdatedAt)
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum AddTitleColumn {
    UpdatedAt,
}

#[cfg(test)]
mod tests {
    use crate::{
        m20221017_030528_title::Title, m20221106_033801_add_updated_at_to_title::AddTitleColumn,
    };
    use sea_orm_migration::prelude::*;

    #[test]
    fn test() {
        // test query string
        let add_column_query = Table::alter()
            .table(Title::Table)
            .add_column(
                ColumnDef::new(AddTitleColumn::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(sea_query::Value::ChronoDateTimeWithTimeZone(Some(
                        Box::new(chrono::Local::now().into()),
                    ))),
            )
            .to_owned();
        assert_eq!(
            add_column_query.to_string(PostgresQueryBuilder),
            [
                r#"ALTER TABLE "title""#,
                r#"ADD COLUMN "updated_at" date NOT NULL,"#,
            ]
            .join(" ")
        );
        let drop_column_query = Table::alter()
            .table(Title::Table)
            .drop_column(AddTitleColumn::UpdatedAt)
            .to_owned();
        assert_eq!(
            drop_column_query.to_string(PostgresQueryBuilder),
            [r#"ALTER TABLE "title""#, r#"DROP COLUMN "updated_at","#,].join(" ")
        );
    }
}
