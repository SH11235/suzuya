use serde::{Deserialize, Serialize};

// entity
#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ItemModel {
    pub id: String,
    pub name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>, // 種類数
    pub illust_status: String,
    // pic：person in charge
    pub pic_illust_id: Option<String>, // from user 「イラスト担当者」
    pub design_status: String,
    pub pic_design_id: Option<String>, // from user 「デザイン担当者」
    pub maker_id: Option<String>,      // from maker
    pub retail_price: Option<i32>,     // 上代
    pub double_check_person_id: Option<String>, // from user 社員名
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkerModel {
    pub id: String,
    pub name: String,
    pub deleted: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MakerModel {
    pub id: String,
    pub code_name: String,
    pub deleted: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NameOptionIdPair {
    pub name: String,
    pub id: Option<String>,
}

// Request Parameter for PUT: /api/item
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct RequestPutTitleInfo {
    pub release_date: Option<String>,
    pub reservation_start_date: Option<String>,
    pub reservation_deadline: Option<String>,
    pub order_date_to_maker: Option<String>,
    pub title: String,
    pub project_type: String,
    pub items: Vec<PutItem>,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PutItem {
    pub id: String,
    pub name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>,
    pub illust_status: String,
    pub pic_illust_id: Option<String>,
    pub design_status: String,
    pub pic_design_id: Option<String>,
    pub maker_id: Option<String>,
    pub retail_price: Option<i32>,
    pub double_check_person_id: Option<String>,
}

// Response for GET /api/item/{title_id}
#[derive(Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GetItemInfoByTitleId {
    pub items: Vec<ItemModel>,
    pub workers: Vec<WorkerModel>,
    pub makers: Vec<MakerModel>,
    pub release_date: Option<String>,
    pub reservation_start_date: Option<String>,
    pub reservation_deadline: Option<String>,
    pub order_date_to_maker: Option<String>,
    pub project_type: String,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
    pub title: String,
}
