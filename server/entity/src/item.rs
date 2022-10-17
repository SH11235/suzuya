use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub release_date: Option<DateTimeWithTimeZone>,
    pub reservation_start_date: Option<DateTimeWithTimeZone>,
    pub reservation_deadline: Option<DateTimeWithTimeZone>,
    pub order_date: Option<DateTimeWithTimeZone>,
    pub title_id: Uuid,
    pub project_type: String,
    pub last_updated: DateTimeWithTimeZone,
    pub name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>,
    pub illust_status: String,
    pub pic_illust_id: Option<Uuid>,
    pub design_status: String,
    pub pic_design_id: Option<Uuid>,
    pub maker_id: Option<Uuid>,
    pub retail_price: Option<i32>,
    pub double_check_person_id: Option<Uuid>,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
    pub deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::DoubleCheckPersonId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker3,
    #[sea_orm(
        belongs_to = "super::maker::Entity",
        from = "Column::MakerId",
        to = "super::maker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Maker,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::PicDesignId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker2,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::PicIllustId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker1,
    #[sea_orm(
        belongs_to = "super::title::Entity",
        from = "Column::TitleId",
        to = "super::title::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Title,
}

impl Related<super::maker::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Maker.def()
    }
}

impl Related<super::title::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Title.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
