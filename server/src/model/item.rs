use chrono::{DateTime, Utc};
use entity::{item, maker, worker};
use sea_orm::prelude::{DateTimeUtc, DateTimeWithTimeZone, Uuid};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult)]
pub struct SelectResult {
    pub id: Uuid,
    pub release_date: Option<DateTime<Utc>>,
    pub reservation_start_date: Option<DateTime<Utc>>,
    pub reservation_deadline: Option<DateTime<Utc>>,
    pub order_date_to_maker: Option<DateTime<Utc>>,
    pub title_id: Uuid,
    pub title: String,
    pub project_type: String,
    pub name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>,
    pub illust_status: String,
    pub pic_illust: Option<String>,
    pub design_status: String,
    pub pic_design: Option<String>,
    pub maker_code: Option<String>,
    pub retail_price: Option<i32>,
    pub double_check_person: Option<String>,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
}

#[derive(Clone, Debug, FromQueryResult, Serialize)]
pub struct YearMonthList {
    pub yyyymm: String,
    pub year: String,
    pub month: String,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputNewItem {
    pub title_id: Uuid,
    pub title_name: String,
    pub release_date: Option<DateTimeUtc>,
    pub delivery_date: Option<DateTimeUtc>,
    pub list_submission_date: Option<DateTimeUtc>,
    pub reservation_start_date: Option<DateTimeUtc>,
    pub reservation_deadline: Option<DateTimeUtc>,
    pub order_date_to_maker: Option<DateTimeUtc>,
    pub project_type: String,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
    pub items: Vec<RequestItems>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ItemsPutRequest {
    pub release_date: Option<DateTimeUtc>,
    pub delivery_date: Option<DateTimeUtc>,
    pub list_submission_date: Option<DateTimeUtc>,
    pub reservation_start_date: Option<DateTimeUtc>,
    pub reservation_deadline: Option<DateTimeUtc>,
    pub order_date_to_maker: Option<DateTimeUtc>,
    pub title_id: Uuid,
    pub title_name: String,
    pub project_type: String,
    pub items: Vec<RequestItems>,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RequestItems {
    pub id: Uuid,
    pub name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>,
    pub illust_status: String,
    pub pic_illust_id: Option<Uuid>,
    pub design_status: String,
    pub pic_design_id: Option<Uuid>,
    pub maker_id: Option<Uuid>,
    pub retail_price: Option<i32>,
    pub resubmission: bool,
    pub double_check_person_id: Option<Uuid>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemNewResponse {
    pub workers: Vec<worker::Model>,
    pub makers: Vec<maker::Model>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemEditResponse {
    pub items: Vec<item::Model>,
    pub workers: Vec<worker::Model>,
    pub makers: Vec<maker::Model>,
    pub release_date: Option<DateTimeWithTimeZone>,
    pub delivery_date: Option<DateTimeWithTimeZone>,
    pub list_submission_date: Option<DateTimeWithTimeZone>,
    pub reservation_start_date: Option<DateTimeWithTimeZone>,
    pub reservation_deadline: Option<DateTimeWithTimeZone>,
    pub order_date_to_maker: Option<DateTimeWithTimeZone>,
    pub updated_at: DateTimeWithTimeZone,
    pub title: String,
    pub project_type: String,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ItemListResponse {
    pub yyyymm: String,
    pub year: String,
    pub month: String,
    pub title_count: usize,
    pub item_count: usize,
    pub title_list: Vec<TitleWithItems>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TitleWithItems {
    pub id: Uuid,
    pub name: String,
    pub release_date: Option<DateTimeWithTimeZone>,
    pub delivery_date: Option<DateTimeWithTimeZone>,
    pub list_submission_date: Option<DateTimeWithTimeZone>,
    pub reservation_start_date: Option<DateTimeWithTimeZone>,
    pub reservation_deadline: Option<DateTimeWithTimeZone>,
    pub order_date_to_maker: Option<DateTimeWithTimeZone>,
    pub updated_at: DateTimeWithTimeZone,
    pub project_type: String,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
    pub items: Vec<ItemWithMakerAndWorker>,
}

#[derive(Clone, Debug, FromQueryResult, Serialize)]
pub struct ItemWithMakerAndWorker {
    pub id: Uuid,
    pub name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>,
    pub illust_status: String,
    pub pic_illust_id: Option<Uuid>,
    pub pic_illust: Option<String>,
    pub design_status: String,
    pub pic_design_id: Option<Uuid>,
    pub pic_design: Option<String>,
    pub maker_id: Option<Uuid>,
    pub maker_code: Option<String>,
    pub retail_price: Option<i32>,
    pub resubmission: bool,
    pub double_check_person_id: Option<Uuid>,
    pub double_check_person: Option<String>,
    pub line: String,
    pub rough_coordinator_id: Option<Uuid>,
    pub rough_coordinator: Option<String>,
    pub rough_check_person_id: Option<Uuid>,
    pub rough_check_person: Option<String>,
    pub line_art_coordinator_id: Option<Uuid>,
    pub line_art_coordinator: Option<String>,
    pub line_art_check_person_id: Option<Uuid>,
    pub line_art_check_person: Option<String>,
    pub coloring_coordinator_id: Option<Uuid>,
    pub coloring_coordinator: Option<String>,
    pub coloring_check_person_id: Option<Uuid>,
    pub coloring_check_person: Option<String>,
    pub design_coordinator_id: Option<Uuid>,
    pub design_coordinator: Option<String>,
    pub design_check_person_id: Option<Uuid>,
    pub design_check_person: Option<String>,
    pub submission_data_coordinator_id: Option<Uuid>,
    pub submission_data_coordinator: Option<String>,
    pub submission_data_check_person_id: Option<Uuid>,
    pub submission_data_check_person: Option<String>,
    pub announcement_materials_coordinator_id: Option<Uuid>,
    pub announcement_materials_coordinator: Option<String>,
    pub announcement_materials_check_person_id: Option<Uuid>,
    pub announcement_materials_check_person: Option<String>,
    pub jan_coordinator_id: Option<Uuid>,
    pub jan_coordinator: Option<String>,
    pub jan_check_person_id: Option<Uuid>,
    pub jan_check_person: Option<String>,
}

#[derive(Clone, Debug, FromQueryResult, Serialize)]
pub struct TitleFiltered {
    pub id: Uuid,
    pub name: String,
    pub release_date: Option<DateTimeWithTimeZone>,
    pub delivery_date: Option<DateTimeWithTimeZone>,
    pub list_submission_date: Option<DateTimeWithTimeZone>,
    pub reservation_start_date: Option<DateTimeWithTimeZone>,
    pub reservation_deadline: Option<DateTimeWithTimeZone>,
    pub order_date_to_maker: Option<DateTimeWithTimeZone>,
    pub updated_at: DateTimeWithTimeZone,
    pub project_type: String,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
}
