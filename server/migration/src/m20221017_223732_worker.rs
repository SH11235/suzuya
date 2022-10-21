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
                    .table(Worker::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Worker::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Uuid::new_v4()),
                    )
                    .col(ColumnDef::new(Worker::Name).string().not_null())
                    .col(
                        ColumnDef::new(Worker::Deleted)
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
            .drop_table(Table::drop().table(Worker::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Worker {
    Table,
    Id,
    Name,
    Deleted,
}

#[cfg(test)]
mod tests {
    use super::Worker;
    use sea_orm_migration::prelude::*;

    #[test]
    fn test() {
        // test query string
        let table = Table::create()
            .table(Worker::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Worker::Id).uuid().not_null().primary_key(), // .default(Uuid::new_v4()), // comment out for testing
            )
            .col(ColumnDef::new(Worker::Name).string().not_null())
            .col(
                ColumnDef::new(Worker::Deleted)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .to_owned();
        assert_eq!(
            table.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "worker" ("#,
                r#""id" uuid NOT NULL PRIMARY KEY,"#,
                r#""name" varchar NOT NULL,"#,
                r#""deleted" bool NOT NULL DEFAULT FALSE"#,
                r#")"#,
            ]
            .join(" ")
        );
    }
}
