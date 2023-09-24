use serde::{Deserialize, Serialize};

use super::common::{ItemModel, MakerModel, WorkerModel};

// Request Parameter for PUT: /api/item
#[derive(Debug, PartialEq, Serialize)]
pub struct RequestPutTitleInfo {
    pub release_date: Option<String>,
    pub reservation_start_date: Option<String>,
    pub reservation_deadline: Option<String>,
    pub order_date_to_maker: Option<String>,
    pub title_id: String,
    pub title_name: String,
    pub project_type: String,
    pub items: Vec<ItemRegisterParams>,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ItemRegisterParams {
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
    pub resubmission: bool,
    pub line: String,
}

// Response for GET /api/item_new
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ItemNewResponse {
    pub workers: Vec<WorkerModel>,
    pub makers: Vec<MakerModel>,
}

// Request for POST /api/item_new
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ItemNewRequest {
    pub id: String,
    pub name: String,
    pub release_date: Option<String>, // DateTimeWithTimeZone
    pub reservation_start_date: Option<String>, // DateTimeWithTimeZone
    pub reservation_deadline: Option<String>, // DateTimeWithTimeZone
    pub order_date_to_maker: Option<String>, // DateTimeWithTimeZone
    pub updated_at: String,           // DateTimeWithTimeZone
    pub project_type: String,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
    pub items: Vec<ItemWithMakerAndWorker>,
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
    pub updated_at: String,
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
    pub resubmission: bool,
    pub line: String,
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
    Resubmission,
    Line,
}

#[derive(Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TitleState {
    pub id: String,
    pub release_date: Option<String>,
    pub reservation_start_date: Option<String>,
    pub reservation_deadline: Option<String>,
    pub order_date_to_maker: Option<String>,
    pub updated_at: String,
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

// /api/item_listのレスポンス
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ItemListResponse {
    pub year_month_list: Vec<YearMonth>,
    pub year_month_title_list: Vec<YearMonthTitleList>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct YearMonthTitleList {
    pub yyyymm: String,
    pub year: String,
    pub month: String,
    pub title_list: Vec<TitleWithItems>,
    pub title_count: usize,
    pub item_count: usize,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TitleWithItems {
    pub id: String,
    pub name: String,
    pub release_date: Option<String>, // DateTimeWithTimeZone
    pub reservation_start_date: Option<String>, // DateTimeWithTimeZone
    pub reservation_deadline: Option<String>, // DateTimeWithTimeZone
    pub order_date_to_maker: Option<String>, // DateTimeWithTimeZone
    pub updated_at: String,           // DateTimeWithTimeZone
    pub project_type: String,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
    pub items: Vec<ItemWithMakerAndWorker>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ItemWithMakerAndWorker {
    pub id: String,
    pub name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>,
    pub illust_status: String,
    pub pic_illust_id: Option<String>,
    pub pic_illust: Option<String>,
    pub design_status: String,
    pub pic_design_id: Option<String>,
    pub pic_design: Option<String>,
    pub maker_id: Option<String>,
    pub maker_code: Option<String>,
    pub retail_price: Option<i32>,
    pub resubmission: bool,
    pub line: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct YearMonth {
    pub yyyymm: String,
    pub year: String,
    pub month: String,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct YearMonthState {
    pub year_month_list: Vec<YearMonth>,
    pub selected_yyyymm: String,
    pub title_count: usize,
    pub item_count: usize,
}
