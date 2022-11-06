use crate::m20221017_224042_item::Item;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Item::Table)
                    .add_column(
                        ColumnDef::new(AddItemColumn::Resubmission)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .add_column(
                        ColumnDef::new(AddItemColumn::Line)
                            .string()
                            .not_null()
                            .default("未対応"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Item::Table)
                    .drop_column(AddItemColumn::Resubmission)
                    .drop_column(AddItemColumn::Line)
                    .to_owned(),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        m20221017_224042_item::Item,
        m20221105_071832_add_resubmission_line_to_item::AddItemColumn,
    };
    use sea_orm_migration::prelude::*;

    #[test]
    fn test() {
        // test query string
        let add_column_query = Table::alter()
            .table(Item::Table)
            .add_column(
                ColumnDef::new(AddItemColumn::Resubmission)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .add_column(
                ColumnDef::new(AddItemColumn::Line)
                    .string()
                    .not_null()
                    .default("未対応"),
            )
            .to_owned();
        assert_eq!(
            add_column_query.to_string(PostgresQueryBuilder),
            [
                r#"ALTER TABLE "item""#,
                r#"ADD COLUMN "resubmission" bool NOT NULL DEFAULT FALSE,"#,
                r#"ADD COLUMN "line" varchar NOT NULL DEFAULT '未対応'"#,
            ]
            .join(" ")
        );
        let drop_column_query = Table::alter()
            .table(Item::Table)
            .drop_column(AddItemColumn::Resubmission)
            .drop_column(AddItemColumn::Line)
            .to_owned();
        assert_eq!(
            drop_column_query.to_string(PostgresQueryBuilder),
            [
                r#"ALTER TABLE "item""#,
                r#"DROP COLUMN "resubmission","#,
                r#"DROP COLUMN "line""#,
            ]
            .join(" ")
        );
    }
}

#[derive(Iden)]
enum AddItemColumn {
    Resubmission,
    Line,
}
