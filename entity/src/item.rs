use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub name: String,
    pub product_code: Option<String>,
    pub release_date: Option<DateTimeLocal>, // 発売日
    pub reservation_start_date: Option<DateTimeLocal>, // 予約開始日(BtoBおよびBtoC)
    pub reservation_deadline: Option<DateTimeLocal>, // 予約締切日
    pub order_date: Option<DateTimeLocal>,   // メーカーへの発注日
    pub sku: Option<i32>,                    // 種類数
    pub illust_status: String,
    pub design_status: String,
    pub last_updated: DateTimeLocal, // 最終更新日（ステータス変更時）
    pub retail_price: Option<i32>,   // 上代
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>, // 備考
    // 外部キー
    pub maker_id: Option<i32>, // from maker
    // pic：person in charge
    pub pic_illust_id: Option<i32>, // from user 「イラスト担当者」
    pub pic_design_id: Option<i32>, // from user 「デザイン担当者」
    pub double_check_person_id: Option<i32>, // from user 社員名
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::maker::Entity",
        from = "Column::MakerId",
        to = "super::maker::Column::Id",
    )]
    Maker,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::DoubleCheckPersonId",
        to = "super::user::Column::Id",
    )]
    User1,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::PicIllustId",
        to = "super::user::Column::Id",
    )]
    User2,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::PicDesignId",
        to = "super::user::Column::Id",
    )]
    User3,
}

impl Related<super::maker::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Maker.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User1.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
