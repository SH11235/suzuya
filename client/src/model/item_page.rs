use serde::{Deserialize, Serialize};

use super::common::{ItemModel, MakerModel, WorkerModel};

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
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ItemState {
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
    pub is_saved: bool,
}

pub enum ItemInfo {
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
}

#[derive(Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TitleState {
    pub id: String,
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

pub enum TitleInfo {
    Title,
    ReleaseDate,
    ReservationStartDate,
    ReservationDeadline,
    OrderDateToMaker,
    ProjectType,
    CatalogStatus,
    AnnouncementStatus,
    Remarks,
}

// /api/itemへのリクエストパラメータ
#[derive(Debug, Serialize)]
pub struct PutAPIItemRequest {
    pub release_date: Option<String>,
    pub reservation_start_date: Option<String>,
    pub reservation_deadline: Option<String>,
    pub order_date_to_maker: Option<String>,
    pub title_id: String,
    pub title_name: String,
    pub project_type: String,
    pub items: Vec<PutItemInfo>,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PutItemInfo {
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
