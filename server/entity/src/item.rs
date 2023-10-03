//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title_id: Uuid,
    pub name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>,
    pub illust_status: String,
    pub pic_illust_id: Option<Uuid>,
    pub design_status: String,
    pub pic_design_id: Option<Uuid>,
    pub maker_id: Option<Uuid>,
    pub retail_price: Option<i32>,
    pub double_check_person_id: Option<Uuid>, // 廃止
    pub deleted: bool,
    pub resubmission: bool,
    pub line: String,
    pub rough_coordinator_id: Option<Uuid>,
    pub rough_check_person_id: Option<Uuid>,
    pub line_art_coordinator_id: Option<Uuid>,
    pub line_art_check_person_id: Option<Uuid>,
    pub coloring_coordinator_id: Option<Uuid>,
    pub coloring_check_person_id: Option<Uuid>,
    pub design_coordinator_id: Option<Uuid>,
    pub design_check_person_id: Option<Uuid>,
    pub submission_data_coordinator_id: Option<Uuid>,
    pub submission_data_check_person_id: Option<Uuid>,
    pub announcement_materials_coordinator_id: Option<Uuid>,
    pub announcement_materials_check_person_id: Option<Uuid>,
    pub jan_coordinator_id: Option<Uuid>,
    pub jan_check_person_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::AnnouncementMaterialsCheckPersonId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker17,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::AnnouncementMaterialsCoordinatorId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker16,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::ColoringCheckPersonId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker15,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::ColoringCoordinatorId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker14,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::DesignCheckPersonId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker13,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::DesignCoordinatorId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker12,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::DoubleCheckPersonId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker11,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::JanCheckPersonId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker10,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::JanCoordinatorId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker9,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::LineArtCheckPersonId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker8,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::LineArtCoordinatorId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker7,
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
    Worker6,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::PicIllustId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker5,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::RoughCheckPersonId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker4,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::RoughCoordinatorId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker3,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::SubmissionDataCheckPersonId",
        to = "super::worker::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Worker2,
    #[sea_orm(
        belongs_to = "super::worker::Entity",
        from = "Column::SubmissionDataCoordinatorId",
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
