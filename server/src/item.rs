use std::fmt::Debug;

use crate::setting::{
    announce_status_list, catalog_status_list, design_status_list, illust_status_list,
    project_type_list, AppState, StatusName, DEFAULT_ITEMS_PER_PAGE, ITME_INPUT_NUM,
};
use actix_web::{error, get, post, put, web, Error, HttpResponse, Result};
use chrono::{DateTime, FixedOffset, Utc};
use entity::item::Entity as Item;
use entity::maker::Entity as Maker;
use entity::title::Entity as Title;
use entity::worker::Entity as Worker;
use entity::{item, maker, title, worker};
use sea_orm::prelude::{DateTimeUtc, DateTimeWithTimeZone, Uuid};
use sea_orm::{entity::*, query::*};
use sea_orm::{DbBackend, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ItemListQuery {
    year: Option<String>,
    month: Option<String>,
    page: Option<usize>,
    items_per_page: Option<usize>,
}

#[derive(Debug, FromQueryResult)]
struct SelectResult {
    id: Uuid,
    release_date: Option<DateTime<Utc>>,
    reservation_start_date: Option<DateTime<Utc>>,
    reservation_deadline: Option<DateTime<Utc>>,
    order_date_to_maker: Option<DateTime<Utc>>,
    title: String,
    project_type: String,
    name: String,
    product_code: Option<String>,
    sku: Option<i32>,
    illust_status: String,
    pic_illust: Option<String>,
    design_status: String,
    pic_design: Option<String>,
    maker_code: Option<String>,
    retail_price: Option<i32>,
    double_check_person: Option<String>,
    catalog_status: String,
    announcement_status: String,
    remarks: Option<String>,
}

#[derive(Debug, FromQueryResult, Serialize)]
struct YearMonthList {
    yyyymm: String,
    year: String,
    month: String,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct InputNewItem {
    title: String,
    release_date: Option<DateTime<Utc>>,
    reservation_start_date: Option<DateTime<Utc>>,
    reservation_deadline: Option<DateTime<Utc>>,
    order_date_to_maker: Option<DateTime<Utc>>,
    name_list: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct JsonItems {
    release_date: Option<DateTimeUtc>,
    reservation_start_date: Option<DateTimeUtc>,
    reservation_deadline: Option<DateTimeUtc>,
    order_date_to_maker: Option<DateTimeUtc>,
    title_id: Uuid,
    title_name: String,
    project_type: String,
    items: Vec<RequestItems>,
    catalog_status: String,
    announcement_status: String,
    remarks: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct RequestItems {
    id: Option<Uuid>,
    name: String,
    product_code: Option<String>,
    sku: Option<i32>,
    illust_status: String,
    pic_illust_id: Option<Uuid>,
    design_status: String,
    pic_design_id: Option<Uuid>,
    maker_id: Option<Uuid>,
    retail_price: Option<i32>,
    double_check_person_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ItemEdit {
    items: Vec<item::Model>,
    workers: Vec<worker::Model>,
    makers: Vec<maker::Model>,
    project_type_list: Vec<StatusName>,
    illust_status_list: Vec<StatusName>,
    design_status_list: Vec<StatusName>,
    release_date: Option<DateTimeWithTimeZone>,
    reservation_start_date: Option<DateTimeWithTimeZone>,
    reservation_deadline: Option<DateTimeWithTimeZone>,
    order_date_to_maker: Option<DateTimeWithTimeZone>,
    title: String,
    project_type: String,
    catalog_status: String,
    announcement_status: String,
    remarks: Option<String>,
    catalog_status_list: Vec<StatusName>,
    announce_status_list: Vec<StatusName>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DateInfo {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

#[get("/item")]
async fn item_list(
    data: web::Data<AppState>,
    query: web::Query<ItemListQuery>,
) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let conn = &data.conn;
    let page = query.page.unwrap_or(1);
    let year_param = query.year.as_ref();
    let month_param = query.month.as_ref();
    let where_str = if year_param.is_some() && month_param.is_some() {
        format!(
            "WHERE to_char(\"items\".\"release_date\", 'YYYY/MM') = '{}/{}'",
            year_param.unwrap(),
            month_param.unwrap()
        )
    } else {
        "WHERE release_date IS NULL".to_string()
    };

    let sql_select = r#"
            "items"."id",
            "items"."release_date",
            "items"."reservation_start_date",
            "items"."reservation_deadline",
            "items"."order_date_to_maker",
            "items"."title",
            "items"."project_type",
            "items"."name",
            "items"."product_code",
            "items"."sku",
            "items"."illust_status",
            "pics_illust"."name" AS "pic_illust",
            "items"."design_status",
            "pics_design"."name" AS "pic_design",
            "makers"."code_name" AS "maker_code",
            "items"."retail_price",
            "workers"."name" AS "double_check_person",
            "items"."catalog_status",
            "items"."announcement_status",
            "items"."remarks"
        FROM
            "items"
            LEFT JOIN "makers" ON "items"."maker_id" = "makers"."id"
            LEFT JOIN "workers" AS "pics_illust" ON "items"."pic_illust_id" = "pics_illust"."id"
            LEFT JOIN "workers" AS "pics_design" ON "items"."pic_design_id" = "pics_design"."id"
            LEFT JOIN "workers" ON "items"."double_check_person_id" = "workers"."id"
    "#;

    let sql_order = r#"
        ORDER BY
            "items"."title" ASC, "items"."id" ASC
    "#;

    let sql = sql_select.to_string() + &where_str + sql_order;

    let items_per_page = query.items_per_page.unwrap_or(DEFAULT_ITEMS_PER_PAGE);
    let paginator = SelectResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        &sql,
        vec![],
    ))
    .paginate(conn, items_per_page);
    let num_pages = paginator.num_pages().await.ok().unwrap();

    let datas = paginator
        .fetch_page(page - 1)
        .await
        .expect("could not retrieve datas");
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    struct ViewData {
        id: Uuid,
        release_date: Option<String>,
        reservation_start_date: Option<String>, // 予約開始日(BtoBおよびBtoC)
        reservation_deadline: Option<String>,   // 予約締切日
        order_date_to_maker: Option<String>,    // メーカーへの発注日
        title: String,
        project_type: String,
        name: String,
        product_code: Option<String>,
        sku: Option<i32>, // 種類数
        illust_status: String,
        pic_illust: Option<String>, // from worker 「イラスト担当者」
        design_status: String,
        pic_design: Option<String>, // from worker 「デザイン担当者」
        maker_code: Option<String>, // from maker
        retail_price: Option<i32>,  // 上代
        double_check_person: Option<String>, // from worker 「社員名」
        catalog_status: String,
        announcement_status: String,
        remarks: Option<String>, // 備考
    }

    let view_datas = datas
        .iter()
        .map(|item| ViewData {
            id: item.id,
            release_date: {
                if let Some(date) = item.release_date {
                    Some(date_to_string(&date))
                } else {
                    None
                }
            },
            reservation_start_date: {
                if let Some(date) = item.reservation_start_date {
                    Some(date_to_string(&date))
                } else {
                    None
                }
            },
            reservation_deadline: {
                if let Some(date) = item.reservation_deadline {
                    Some(date_to_string(&date))
                } else {
                    None
                }
            },
            order_date_to_maker: {
                if let Some(date) = item.order_date_to_maker {
                    Some(date_to_string(&date))
                } else {
                    None
                }
            },
            title: item.title.clone(),
            project_type: item.project_type.clone(),
            name: item.name.clone(),
            product_code: item.product_code.clone(),
            sku: item.sku,
            illust_status: item.illust_status.clone(),
            pic_illust: item.pic_illust.clone(),
            design_status: item.design_status.clone(),
            pic_design: item.pic_design.clone(),
            maker_code: item.maker_code.clone(),
            retail_price: item.retail_price,
            double_check_person: item.double_check_person.clone(),
            catalog_status: item.catalog_status.clone(),
            announcement_status: item.announcement_status.clone(),
            remarks: item.remarks.clone(),
        })
        .collect::<Vec<ViewData>>();

    let year_month_sql = "
        SELECT
            to_char(release_date, 'YYYY/MM') as yyyymm,
            to_char(release_date, 'YYYY') as year,
            to_char(release_date, 'MM') as month
        FROM items
        WHERE release_date is not NULL
        GROUP BY yyyymm, year, month
        ORDER BY yyyymm DESC NULLS FIRST;
    ";
    let year_month_list = YearMonthList::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        year_month_sql,
        vec![],
    ))
    .all(conn)
    .await
    .expect("could not find items.");

    let mut ctx = tera::Context::new();
    let h1 = "アイテム";
    let path = "item";
    ctx.insert("view_datas", &view_datas);
    ctx.insert("year_month_list", &year_month_list);
    ctx.insert("page", &page);
    ctx.insert("h1", &h1);
    ctx.insert("path", &path);
    ctx.insert("items_per_page", &items_per_page);
    ctx.insert("num_pages", &num_pages);

    let body = template
        .render("item/item_list.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/new_item")]
async fn new_item(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let mut ctx = tera::Context::new();
    let h4 = "アイテム登録";
    let path = "item";
    let input_id_list: Vec<i32> = (1..ITME_INPUT_NUM + 1).collect();
    let spacer = ",";

    ctx.insert("h4", &h4);
    ctx.insert("path", &path);
    ctx.insert("input_id_list", &input_id_list);
    ctx.insert("spacer", &spacer);
    let body = template
        .render("item/new_item.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/new_item")]
async fn create_items(
    data: web::Data<AppState>,
    post_data: web::Json<InputNewItem>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = post_data.into_inner();
    let name_list = form.name_list;

    // title登録
    let title = title::ActiveModel {
        name: Set(form.title),
        deleted: Set(false),
        ..Default::default()
    }
    .insert(conn)
    .await
    .expect("could not insert title.");

    // item登録
    let title_id = title.id;
    for item_name in name_list.iter() {
        item::ActiveModel {
            title_id: Set(title_id),
            name: Set(item_name.to_string()),
            ..Default::default()
        }
        .insert(conn)
        .await
        .expect("could not insert item");
    }

    Ok(HttpResponse::Found()
        .append_header(("location", "/new_item"))
        .finish())
}

#[get("/api/item/{title_id}")]
async fn api_edit_items(
    data: web::Data<AppState>,
    title_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let title = Title::find_by_id(*title_id)
        .one(conn)
        .await
        .expect("could not find title.")
        .unwrap();

    let title_name = title.name;
    let project_type = title.project_type;
    let catalog_status = title.catalog_status;
    let announcement_status = title.announcement_status;
    let remarks = title.remarks;

    let items = Item::find()
        .order_by_asc(item::Column::Id)
        .filter(item::Column::TitleId.eq(title_id.to_owned()))
        .all(conn)
        .await
        .expect("could not find items by title_id.");

    let workers = Worker::find()
        .order_by_asc(worker::Column::Id)
        .filter(worker::Column::Deleted.eq(false))
        .all(conn)
        .await
        .expect("could not find workers.");

    let makers = Maker::find()
        .filter(maker::Column::Deleted.eq(false))
        .order_by_asc(maker::Column::Id)
        .all(conn)
        .await
        .expect("could not find makers.");

    let project_type_list = project_type_list();
    let illust_status_list = illust_status_list();
    let design_status_list = design_status_list();
    let catalog_status_list = catalog_status_list();
    let announce_status_list = announce_status_list();

    let release_date: Option<DateTimeWithTimeZone> = title.release_date;
    let reservation_start_date: Option<DateTimeWithTimeZone> = title.reservation_start_date;
    let reservation_deadline: Option<DateTimeWithTimeZone> = title.reservation_deadline;
    let order_date_to_maker: Option<DateTimeWithTimeZone> = title.order_date_to_maker;

    Ok(HttpResponse::Ok().json(ItemEdit {
        items: items,
        workers: workers,
        makers: makers,
        project_type_list: project_type_list,
        illust_status_list: illust_status_list,
        design_status_list: design_status_list,
        release_date,
        reservation_start_date,
        reservation_deadline,
        order_date_to_maker,
        title: title_name,
        project_type,
        catalog_status,
        announcement_status,
        remarks,
        catalog_status_list,
        announce_status_list,
    }))
}

#[put("/api/item")]
async fn update_items(
    data: web::Data<AppState>,
    post_data: web::Json<JsonItems>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let put_data = post_data.into_inner();
    let title_id = put_data.title_id.clone();

    // title_id存在確認
    Title::find_by_id(title_id)
        .one(conn)
        .await
        .expect("could not find title.")
        .unwrap()
        .into_active_model();
    // title更新
    title::ActiveModel {
        id: Set(title_id),
        name: Set(put_data.title_name),
        release_date: match put_data.release_date {
            Some(release_date) => Set(Some(utc_date_time_to_jst(&release_date))),
            None => Set(None),
        },
        reservation_start_date: match put_data.reservation_start_date {
            Some(reservation_start_date) => {
                Set(Some(utc_date_time_to_jst(&reservation_start_date)))
            }
            None => Set(None),
        },
        reservation_deadline: match put_data.reservation_deadline {
            Some(reservation_deadline) => Set(Some(utc_date_time_to_jst(&reservation_deadline))),
            None => Set(None),
        },
        order_date_to_maker: match put_data.order_date_to_maker {
            Some(order_date_to_maker) => Set(Some(utc_date_time_to_jst(&order_date_to_maker))),
            None => Set(None),
        },
        project_type: Set(put_data.project_type),
        catalog_status: Set(put_data.catalog_status),
        announcement_status: Set(put_data.announcement_status),
        remarks: Set(put_data.remarks),
        deleted: Set(false),
    }
    .update(conn)
    .await
    .expect("could not update title.");

    for item in put_data.items.iter() {
        let item_id = &item.id;

        match item_id {
            Some(item_id) => {
                // item_id存在確認
                Item::find_by_id(*item_id)
                    .one(conn)
                    .await
                    .expect("could not find item.");
                // idあり→UPDATE
                item::ActiveModel {
                    id: Set(*item_id),
                    title_id: Set(put_data.title_id.to_owned()),
                    name: Set(item.name.to_owned()),
                    product_code: Set(item.product_code.to_owned()),
                    sku: Set(item.sku.to_owned()),
                    illust_status: Set(item.illust_status.to_owned()),
                    pic_illust_id: Set(item.pic_illust_id.to_owned()),
                    design_status: Set(item.design_status.to_owned()),
                    pic_design_id: Set(item.pic_design_id.to_owned()),
                    maker_id: Set(item.maker_id),
                    retail_price: Set(item.retail_price.to_owned()),
                    double_check_person_id: Set(item.double_check_person_id.to_owned()),
                    ..Default::default()
                }
                .save(conn)
                .await
                .expect("could not edit items");
            }
            None => {
                // id無し→INSERT
                item::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    title_id: Set(put_data.title_id.to_owned()),
                    name: Set(item.name.to_owned()),
                    product_code: Set(item.product_code.to_owned()),
                    sku: Set(item.sku.to_owned()),
                    illust_status: Set(item.illust_status.to_owned()),
                    pic_illust_id: Set(item.pic_illust_id.to_owned()),
                    design_status: Set(item.design_status.to_owned()),
                    pic_design_id: Set(item.pic_design_id.to_owned()),
                    maker_id: Set(item.maker_id.to_owned()),
                    retail_price: Set(item.retail_price.to_owned()),
                    double_check_person_id: Set(item.double_check_person_id.to_owned()),
                    ..Default::default()
                }
                .insert(conn)
                .await
                .expect("failed to insert items");
            }
        }
    }

    Ok(HttpResponse::Ok().body("put ok"))
}

fn date_to_string(date_time: &DateTime<Utc>) -> String {
    // format document https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html
    let date_time_japan = utc_date_time_to_jst(date_time);
    let day_format = date_time_japan.format("%w").to_string(); // Sunday = 0, Monday = 1, ..., Saturday = 6.
    let day_jp = match &*day_format {
        "0" => "(日)",
        "1" => "(月)",
        "2" => "(火)",
        "3" => "(水)",
        "4" => "(木)",
        "5" => "(金)",
        "6" => "(土)",
        _ => "(-)",
    };
    let month_date = date_time_japan.format("%m/%d").to_string();
    format!("{}{}", month_date, day_jp)
}

fn utc_date_time_to_jst(date_time: &DateTime<Utc>) -> DateTime<FixedOffset> {
    date_time.with_timezone(&FixedOffset::east(9 * 3600))
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::item::date_to_string;

    #[test]
    fn test_date_to_string() {
        let date_time = Utc.ymd(2022, 2, 21).and_hms(16, 0, 0);
        assert_eq!(date_to_string(&date_time), "02/22(火)".to_string());

        let date_time = Utc.ymd(2022, 12, 31).and_hms(11, 0, 0);
        assert_eq!(date_to_string(&date_time), "12/31(土)".to_string());

        let date_time = Utc.ymd(2022, 12, 31).and_hms(16, 0, 0);
        assert_eq!(date_to_string(&date_time), "01/01(日)".to_string());

        let date_time = Utc.ymd(2022, 1, 1).and_hms(0, 0, 0);
        assert_eq!(date_to_string(&date_time), "01/01(土)".to_string());
    }
}
