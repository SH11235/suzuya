use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PutItem {
    pub id: i32,
    pub name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>,
    pub illust_status: String,
    pub pic_illust_id: Option<i32>,
    pub design_status: String,
    pub pic_design_id: Option<i32>,
    pub maker_id: Option<i32>,
    pub retail_price: Option<i32>,
    pub double_check_person_id: Option<i32>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub deleted: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MakerModel {
    pub id: i32,
    pub code_name: String,
    pub deleted: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NameIdPair {
    pub name: String,
    pub id: i32,
}

#[derive(Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GetItem {
    pub items: Vec<ItemModel>,
    pub users: Vec<UserModel>,
    pub makers: Vec<MakerModel>,
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
    pub items: Vec<PutItem>,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
}
