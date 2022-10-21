use crate::m20221017_030456_maker::Maker;
use crate::m20221017_030528_title::Title;
use crate::m20221017_223732_worker::Worker;
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
                    .table(Item::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Item::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Uuid::new_v4()),
                    )
                    .col(ColumnDef::new(Item::TitleId).uuid().not_null())
                    .col(
                        ColumnDef::new(Item::ProjectType)
                            .string()
                            .default("デフォルト")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Item::LastUpdated)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Item::Name).string().not_null())
                    .col(ColumnDef::new(Item::ProductCode).string())
                    .col(ColumnDef::new(Item::Sku).integer())
                    .col(
                        ColumnDef::new(Item::IllustStatus)
                            .string()
                            .default("未着手")
                            .not_null(),
                    )
                    .col(ColumnDef::new(Item::PicIllustId).uuid())
                    .col(
                        ColumnDef::new(Item::DesignStatus)
                            .string()
                            .default("未着手")
                            .not_null(),
                    )
                    .col(ColumnDef::new(Item::PicDesignId).uuid())
                    .col(ColumnDef::new(Item::MakerId).uuid())
                    .col(ColumnDef::new(Item::RetailPrice).integer())
                    .col(ColumnDef::new(Item::DoubleCheckPersonId).uuid())
                    .col(
                        ColumnDef::new(Item::CatalogStatus)
                            .string()
                            .default("未着手")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Item::AnnouncementStatus)
                            .string()
                            .default("未着手")
                            .not_null(),
                    )
                    .col(ColumnDef::new(Item::Remarks).string())
                    .col(
                        ColumnDef::new(Item::Deleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("item_title_id_fkey")
                            .from(Item::Table, Item::TitleId)
                            .to(Title::Table, Title::Id),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("item_maker_id_fkey")
                            .from(Item::Table, Item::MakerId)
                            .to(Maker::Table, Maker::Id),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("item_pic_illust_id_fkey")
                            .from(Item::Table, Item::PicIllustId)
                            .to(Worker::Table, Worker::Id),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("item_pic_design_id_fkey")
                            .from(Item::Table, Item::PicDesignId)
                            .to(Worker::Table, Worker::Id),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("item_double_check_person_id_fkey")
                            .from(Item::Table, Item::DoubleCheckPersonId)
                            .to(Worker::Table, Worker::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // ↑の処理中でindexを作成するとErrorになるので、別途作成する
        manager
            .create_index(
                sea_query::Index::create()
                    .name("item_title_id_idx")
                    .table(Item::Table)
                    .col(Item::TitleId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Item {
    Table,
    Id,
    TitleId,
    ProjectType,
    LastUpdated,
    Name,
    ProductCode,
    Sku,
    IllustStatus,
    PicIllustId,
    DesignStatus,
    PicDesignId,
    MakerId,
    RetailPrice,
    DoubleCheckPersonId,
    CatalogStatus,
    AnnouncementStatus,
    Remarks,
    Deleted,
}

#[cfg(test)]
mod tests {
    use super::Item;
    use crate::{
        m20221017_030456_maker::Maker, m20221017_030528_title::Title,
        m20221017_223732_worker::Worker,
    };
    use sea_orm_migration::prelude::*;

    #[test]
    fn test() {
        // test query string
        let table = Table::create()
            .table(Item::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Item::Id).uuid().not_null().primary_key(), // .default(Uuid::new_v4()),
            )
            .col(ColumnDef::new(Item::TitleId).uuid().not_null())
            .col(
                ColumnDef::new(Item::ProjectType)
                    .string()
                    .default("デフォルト")
                    .not_null(),
            )
            .col(
                ColumnDef::new(Item::LastUpdated)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new(Item::Name).string().not_null())
            .col(ColumnDef::new(Item::ProductCode).string())
            .col(ColumnDef::new(Item::Sku).integer())
            .col(
                ColumnDef::new(Item::IllustStatus)
                    .string()
                    .default("未着手")
                    .not_null(),
            )
            .col(ColumnDef::new(Item::PicIllustId).uuid())
            .col(
                ColumnDef::new(Item::DesignStatus)
                    .string()
                    .default("未着手")
                    .not_null(),
            )
            .col(ColumnDef::new(Item::PicDesignId).uuid())
            .col(ColumnDef::new(Item::MakerId).uuid())
            .col(ColumnDef::new(Item::RetailPrice).integer())
            .col(ColumnDef::new(Item::DoubleCheckPersonId).uuid())
            .col(
                ColumnDef::new(Item::CatalogStatus)
                    .string()
                    .default("未着手")
                    .not_null(),
            )
            .col(
                ColumnDef::new(Item::AnnouncementStatus)
                    .string()
                    .default("未着手")
                    .not_null(),
            )
            .col(ColumnDef::new(Item::Remarks).string())
            .col(
                ColumnDef::new(Item::Deleted)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .foreign_key(
                sea_query::ForeignKey::create()
                    .name("item_title_id_fkey")
                    .from(Item::Table, Item::TitleId)
                    .to(Title::Table, Title::Id),
            )
            .foreign_key(
                sea_query::ForeignKey::create()
                    .name("item_maker_id_fkey")
                    .from(Item::Table, Item::MakerId)
                    .to(Maker::Table, Maker::Id),
            )
            .foreign_key(
                sea_query::ForeignKey::create()
                    .name("item_pic_illust_id_fkey")
                    .from(Item::Table, Item::PicIllustId)
                    .to(Worker::Table, Worker::Id),
            )
            .foreign_key(
                sea_query::ForeignKey::create()
                    .name("item_pic_design_id_fkey")
                    .from(Item::Table, Item::PicDesignId)
                    .to(Worker::Table, Worker::Id),
            )
            .foreign_key(
                sea_query::ForeignKey::create()
                    .name("item_double_check_person_id_fkey")
                    .from(Item::Table, Item::DoubleCheckPersonId)
                    .to(Worker::Table, Worker::Id),
            )
            .to_owned();
        assert_eq!(
            table.to_string(PostgresQueryBuilder),
            [
                r#"CREATE TABLE IF NOT EXISTS "item" ("#,
                r#""id" uuid NOT NULL PRIMARY KEY,"#,
                r#""title_id" uuid NOT NULL,"#,
                r#""project_type" varchar DEFAULT 'デフォルト' NOT NULL,"#,
                r#""last_updated" timestamp with time zone NOT NULL,"#,
                r#""name" varchar NOT NULL,"#,
                r#""product_code" varchar,"#,
                r#""sku" integer,"#,
                r#""illust_status" varchar DEFAULT '未着手' NOT NULL,"#,
                r#""pic_illust_id" uuid,"#,
                r#""design_status" varchar DEFAULT '未着手' NOT NULL,"#,
                r#""pic_design_id" uuid,"#,
                r#""maker_id" uuid,"#,
                r#""retail_price" integer,"#,
                r#""double_check_person_id" uuid,"#,
                r#""catalog_status" varchar DEFAULT '未着手' NOT NULL,"#,
                r#""announcement_status" varchar DEFAULT '未着手' NOT NULL,"#,
                r#""remarks" varchar,"#,
                r#""deleted" bool NOT NULL DEFAULT FALSE,"#,
                r#"CONSTRAINT "item_title_id_fkey" FOREIGN KEY ("title_id") REFERENCES "title" ("id"),"#,
                r#"CONSTRAINT "item_maker_id_fkey" FOREIGN KEY ("maker_id") REFERENCES "maker" ("id"),"#,
                r#"CONSTRAINT "item_pic_illust_id_fkey" FOREIGN KEY ("pic_illust_id") REFERENCES "worker" ("id"),"#,
                r#"CONSTRAINT "item_pic_design_id_fkey" FOREIGN KEY ("pic_design_id") REFERENCES "worker" ("id"),"#,
                r#"CONSTRAINT "item_double_check_person_id_fkey" FOREIGN KEY ("double_check_person_id") REFERENCES "worker" ("id")"#,
                r#")"#,
            ]
            .join(" ")
        );
    }
}
