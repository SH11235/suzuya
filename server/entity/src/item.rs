use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub name: String,
    pub arrival_date: Option<Date>,           // メーカーからの入荷日
    pub reservation_start_date: Option<Date>, // 予約開始日(BtoBおよびBtoC)
    pub reservation_deadline: Option<Date>,   // 予約締切日
    pub order_date: Option<Date>,             // メーカーへの発注日
    pub sku: Option<i32>,                     // 種類数
    pub status: String,
    pub last_updated: Date,        // 最終更新日（ステータス変更時）
    pub retail_price: Option<u32>, // 上代
    pub catalog_status: Option<String>,
    pub announcement_status: Option<String>,
    pub remarks: Option<String>, // 備考
    // 外部キー
    pub maker_id: Option<i32>,               // from maker
    pub category_id: Option<i32>,            // from item_category
    pub pic_id: Option<i32>,                 // from user 「担当者」person in charge
    pub double_check_person_id: Option<i32>, // from user 社員名
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::maker::Entity",
        from = "Column::MakerId",
        to = "super::maker::Column::Id"
    )]
    Maker,

    #[sea_orm(
        belongs_to = "super::item_category::Entity",
        from = "Column::CategoryId",
        to = "super::item_category::Column::Id"
    )]
    ItemCategory,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::PicId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::maker::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Maker.def()
    }
}

impl Related<super::item_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemCategory.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
