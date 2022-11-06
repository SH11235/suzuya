use chrono::{DateTime, Utc};
use entity::{item, maker, worker};
use sea_orm::prelude::{DateTimeUtc, DateTimeWithTimeZone, Uuid};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::setting::StatusName;

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
    pub title: String,
    pub release_date: Option<DateTime<Utc>>,
    pub reservation_start_date: Option<DateTime<Utc>>,
    pub reservation_deadline: Option<DateTime<Utc>>,
    pub order_date_to_maker: Option<DateTime<Utc>>,
    pub name_list: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ItemsPutRequest {
    pub release_date: Option<DateTimeUtc>,
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemEditResponse {
    pub items: Vec<item::Model>,
    pub workers: Vec<worker::Model>,
    pub makers: Vec<maker::Model>,
    pub project_type_list: Vec<StatusName>,
    pub illust_status_list: Vec<StatusName>,
    pub design_status_list: Vec<StatusName>,
    pub release_date: Option<DateTimeWithTimeZone>,
    pub reservation_start_date: Option<DateTimeWithTimeZone>,
    pub reservation_deadline: Option<DateTimeWithTimeZone>,
    pub order_date_to_maker: Option<DateTimeWithTimeZone>,
    pub updated_at: DateTimeWithTimeZone,
    pub title: String,
    pub project_type: String,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
    pub catalog_status_list: Vec<StatusName>,
    pub announce_status_list: Vec<StatusName>,
}

#[derive(Debug, Serialize)]
pub struct ItemListResponse {
    pub year_month_list: Vec<YearMonthList>,
    pub year_month_title_list: Vec<YearMonthTitleList>,
}

#[derive(Debug, Serialize)]
pub struct YearMonthTitleList {
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
}

#[derive(Clone, Debug, FromQueryResult, Serialize)]
pub struct TitleFiltered {
    pub id: Uuid,
    pub name: String,
    pub release_date: Option<DateTimeWithTimeZone>,
    pub reservation_start_date: Option<DateTimeWithTimeZone>,
    pub reservation_deadline: Option<DateTimeWithTimeZone>,
    pub order_date_to_maker: Option<DateTimeWithTimeZone>,
    pub updated_at: DateTimeWithTimeZone,
    pub project_type: String,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
}
