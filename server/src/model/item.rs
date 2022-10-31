use chrono::{DateTime, Utc};
use entity::{item, maker, worker};
use sea_orm::prelude::{DateTimeUtc, DateTimeWithTimeZone, Uuid};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::setting::StatusName;

#[derive(Debug, Deserialize)]
pub struct ItemListQuery {
    pub year: Option<String>,
    pub month: Option<String>,
    pub page: Option<usize>,
    pub items_per_page: Option<usize>,
}

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

#[derive(Debug, FromQueryResult, Serialize)]
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
    pub double_check_person_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemEdit {
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
    pub title: String,
    pub project_type: String,
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>,
    pub catalog_status_list: Vec<StatusName>,
    pub announce_status_list: Vec<StatusName>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ViewData {
    pub id: Uuid,
    pub release_date: Option<String>,
    pub reservation_start_date: Option<String>, // 予約開始日(BtoBおよびBtoC)
    pub reservation_deadline: Option<String>,   // 予約締切日
    pub order_date_to_maker: Option<String>,    // メーカーへの発注日
    pub title: String,
    pub project_type: String,
    pub name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>, // 種類数
    pub illust_status: String,
    pub pic_illust: Option<String>, // from worker 「イラスト担当者」
    pub design_status: String,
    pub pic_design: Option<String>, // from worker 「デザイン担当者」
    pub maker_code: Option<String>, // from maker
    pub retail_price: Option<i32>,  // 上代
    pub double_check_person: Option<String>, // from worker 「社員名」
    pub catalog_status: String,
    pub announcement_status: String,
    pub remarks: Option<String>, // 備考
}

#[derive(Debug, Serialize)]
pub struct ItemListResponse {
    pub year_month_list: Vec<YearMonthList>,
}
