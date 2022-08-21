use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ItemModel {
    pub id: i32,
    pub release_date: Option<String>,           // 発売日
    pub reservation_start_date: Option<String>, // 予約開始日(BtoBおよびBtoC)
    pub reservation_deadline: Option<String>,   // 予約締切日
    pub order_date: Option<String>,             // メーカーへの発注日
    pub title: String,
    pub project_type: String,
    pub last_updated: String, // 最終更新日（ステータス変更時）
    pub name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>, // 種類数
    pub illust_status: String,
    // pic：person in charge
    pub pic_illust_id: Option<i32>, // from user 「イラスト担当者」
    pub design_status: String,
    pub pic_design_id: Option<i32>, // from user 「デザイン担当者」
    pub maker_id: Option<i32>,      // from maker
    pub retail_price: Option<i32>,  // 上代
    pub double_check_person_id: Option<i32>, // from user 社員名
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>, // 備考
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct GetItem {
    pub items: Vec<ItemModel>,
    // pub users: Vec<user::Model>,
    // pub makers: Vec<maker::Model>,
    pub release_date: Option<String>,
    pub reservation_start_date: Option<String>,
    pub reservation_deadline: Option<String>,
    pub order_date: Option<String>,
    pub last_updated: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PostItem {
    pub release_date: Option<String>,
    pub reservation_start_date: Option<String>,
    pub reservation_deadline: Option<String>,
    pub order_date: Option<String>,
    pub title: String,
    pub project_type: String,
    pub items: Vec<ItemModel>,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
}
