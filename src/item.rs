use std::fmt::Debug;

use crate::setting::{
    announce_status_list, catalog_status_list, design_status_list, illust_status_list,
    project_type_list, AppState, Params, DEFAULT_ITEMS_PER_PAGE, ITME_INPUT_NUM,
};
use actix_web::{error, get, post, put, web, Error, HttpRequest, HttpResponse, Result};
use chrono::{DateTime, Local};
use entity::item::Entity as Item;
use entity::maker::Entity as Maker;
use entity::user::Entity as User;
use entity::{item, maker, user};
use sea_orm::{entity::*, prelude::DateTimeLocal, query::*};
use sea_orm::{DbBackend, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct InputNewItem {
    title: String,
    name_list: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct JsonItems {
    release_date: Option<DateTimeLocal>,
    reservation_start_date: Option<DateTimeLocal>,
    reservation_deadline: Option<DateTimeLocal>,
    order_date: Option<DateTimeLocal>,
    title: String,
    project_type: String,
    items: Vec<Items>,
    catalog_status: String,
    announcement_status: String,
    remarks: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Items {
    id: i32,
    name: String,
    product_code: Option<String>,
    sku: Option<i32>,
    illust_status: String,
    pic_illust_id: Option<i32>,
    design_status: String,
    pic_design_id: Option<i32>,
    maker_id: Option<i32>,
    retail_price: Option<i32>,
    double_check_person_id: Option<i32>,
}

#[get("/item")]
async fn item_list(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let conn = &data.conn;

    // get params
    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();

    let page = params.page.unwrap_or(1);
    let items_per_page = params.items_per_page.unwrap_or(DEFAULT_ITEMS_PER_PAGE);
    #[derive(Debug, FromQueryResult)]
    struct SelectResult {
        id: i32,
        release_date: Option<DateTimeLocal>,
        reservation_start_date: Option<DateTimeLocal>,
        reservation_deadline: Option<DateTimeLocal>,
        order_date: Option<DateTimeLocal>,
        title: String,
        project_type: String,
        last_updated: DateTimeLocal,
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
    let paginator = SelectResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
                    "items"."id",
                    "items"."release_date",
                    "items"."reservation_start_date",
                    "items"."reservation_deadline",
                    "items"."order_date",
                    "items"."title",
                    "items"."project_type",
                    "items"."last_updated",
                    "items"."name",
                    "items"."product_code",
                    "items"."sku",
                    "items"."illust_status",
                    "pics_illust"."name" AS "pic_illust",
                    "items"."design_status",
                    "pics_design"."name" AS "pic_design",
                    "makers"."code_name" AS "maker_code",
                    "items"."retail_price",
                    "users"."name" AS "double_check_person",
                    "items"."catalog_status",
                    "items"."announcement_status",
                    "items"."remarks"
                FROM
                    "items"
                    LEFT JOIN "makers" ON "items"."maker_id" = "makers"."id"
                    LEFT JOIN "users" AS "pics_illust" ON "items"."pic_illust_id" = "pics_illust"."id"
                    LEFT JOIN "users" AS "pics_design" ON "items"."pic_design_id" = "pics_design"."id"
                    LEFT JOIN "users" ON "items"."double_check_person_id" = "users"."id"
                ORDER BY
                    "items"."id" ASC
                "#,
        vec![],
    ))
    .paginate(conn, items_per_page);
    let num_pages = paginator.num_pages().await.ok().unwrap();

    let datas = paginator
        .fetch_page(page - 1)
        .await
        .expect("could not retrieve datas");

    println!("{:?}", datas);
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    struct ViewData {
        id: i32,
        release_date: Option<String>,
        reservation_start_date: Option<String>, // 予約開始日(BtoBおよびBtoC)
        reservation_deadline: Option<String>,   // 予約締切日
        order_date: Option<String>,             // メーカーへの発注日
        title: String,
        project_type: String,
        last_updated: String, // 最終更新日（ステータス変更時）
        name: String,
        product_code: Option<String>,
        sku: Option<i32>, // 種類数
        illust_status: String,
        pic_illust: Option<String>, // from user 「イラスト担当者」
        design_status: String,
        pic_design: Option<String>, // from user 「デザイン担当者」
        maker_code: Option<String>, // from maker
        retail_price: Option<i32>,  // 上代
        double_check_person: Option<String>, // from user 「社員名」
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
            order_date: {
                if let Some(date) = item.order_date {
                    Some(date_to_string(&date))
                } else {
                    None
                }
            },
            title: item.title.clone(),
            project_type: item.project_type.clone(),
            last_updated: date_to_string(&item.last_updated),
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

    let mut ctx = tera::Context::new();
    let h1 = "アイテム";
    let path = "item";
    ctx.insert("view_datas", &view_datas);
    ctx.insert("page", &page);
    ctx.insert("h1", &h1);
    ctx.insert("path", &path);
    ctx.insert("items_per_page", &items_per_page);
    ctx.insert("num_pages", &num_pages);

    let body = template
        .render("item_list.html.tera", &ctx)
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
        .render("new_item.html.tera", &ctx)
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

    let date = Local::now();
    let yyyymmddhhmmss = date_to_yyyymmddhhmmss(&date);

    let last_updated = Local::now();
    for item_name in name_list.iter() {
        item::ActiveModel {
            title: Set(format!("{}_{}", form.title.to_owned(), yyyymmddhhmmss)),
            name: Set(item_name.to_string()),
            last_updated: Set(last_updated),
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

#[get("/item/{title}")]
async fn edit_items(
    data: web::Data<AppState>,
    title: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let template = &data.templates;

    let items = Item::find()
        .order_by_asc(item::Column::Id)
        .filter(item::Column::Title.eq(title.to_owned()))
        .all(conn)
        .await
        .expect("could not find items by title.");

    let users = User::find()
        .order_by_asc(user::Column::Id)
        .filter(user::Column::Deleted.eq(false))
        .all(conn)
        .await
        .expect("could not find users.");

    let makers = Maker::find()
        .order_by_asc(maker::Column::Id)
        .all(conn)
        .await
        .expect("could not find users.");

    let mut ctx = tera::Context::new();
    let path = "item";
    let project_type_list = project_type_list();
    let illust_status_list = illust_status_list();
    let design_status_list = design_status_list();
    let catalog_status_list = catalog_status_list();
    let announce_status_list = announce_status_list();

    let release_date: Option<String> = match items[0].release_date {
        Some(release_date) => Some(release_date.format("%Y/%m/%d").to_string()),
        None => None,
    };
    let reservation_start_date: Option<String> = match items[0].reservation_start_date {
        Some(reservation_start_date) => Some(reservation_start_date.format("%Y/%m/%d").to_string()),
        None => None,
    };
    let reservation_deadline: Option<String> = match items[0].reservation_deadline {
        Some(reservation_deadline) => Some(reservation_deadline.format("%Y/%m/%d").to_string()),
        None => None,
    };
    let order_date: Option<String> = match items[0].order_date {
        Some(order_date) => Some(order_date.format("%Y/%m/%d").to_string()),
        None => None,
    };
    let last_updated = items[0]
        .last_updated
        .format("%Y/%m/%d %H:%M:%S")
        .to_string();

    ctx.insert("items", &items);
    ctx.insert("users", &users);
    ctx.insert("makers", &makers);
    ctx.insert("path", &path);
    ctx.insert("project_type_list", &project_type_list);
    ctx.insert("illust_status_list", &illust_status_list);
    ctx.insert("design_status_list", &design_status_list);
    ctx.insert("release_date", &release_date);
    ctx.insert("reservation_start_date", &reservation_start_date);
    ctx.insert("reservation_deadline", &reservation_deadline);
    ctx.insert("order_date", &order_date);
    ctx.insert("last_updated", &last_updated);
    ctx.insert("catalog_status_list", &catalog_status_list);
    ctx.insert("announcement_status_list", &announce_status_list);

    let body = template
        .render("edit_item.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[put("/item")]
async fn update_items(
    data: web::Data<AppState>,
    post_data: web::Json<JsonItems>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let data = post_data.into_inner();
    let last_updated = Local::now();
    for item in data.items.iter() {
        item::ActiveModel {
            id: Set(item.id),
            release_date: Set(data.release_date),
            reservation_start_date: Set(data.reservation_start_date),
            reservation_deadline: Set(data.reservation_deadline),
            order_date: Set(data.order_date),
            title: Set(data.title.to_owned()),
            project_type: Set(data.project_type.to_owned()),
            last_updated: Set(last_updated.to_owned()),
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
            catalog_status: Set(data.catalog_status.to_owned()),
            announcement_status: Set(data.announcement_status.to_owned()),
            remarks: Set(data.remarks.to_owned()),
            ..Default::default()
        }
        .save(conn)
        .await
        .expect("could not edit items");
    }

    Ok(HttpResponse::Ok().body("put ok"))
}

fn date_to_string(date_time: &DateTime<Local>) -> String {
    // format document https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html
    let day_format = date_time.format("%w").to_string(); // Sunday = 0, Monday = 1, ..., Saturday = 6.
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
    let month_date = date_time.format("%m/%d").to_string();
    format!("{}{}", month_date, day_jp)
}

fn date_to_yyyymmddhhmmss(date_time: &DateTime<Local>) -> String {
    date_time.format("%Y%m%d%H%M%S").to_string()
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

    use crate::item::{date_to_string, date_to_yyyymmddhhmmss};

    #[test]
    fn test_date_to_string() {
        let dt: NaiveDateTime =
            NaiveDateTime::parse_from_str("2022/02/22 22:22:22", "%Y/%m/%d %H:%M:%S").unwrap();
        let date_time_local: DateTime<Local> = Local.from_local_datetime(&dt).unwrap();
        assert_eq!(date_to_string(&date_time_local), "02/22(火)".to_string());

        let dt: NaiveDateTime =
            NaiveDateTime::parse_from_str("2022/12/31 23:59:59", "%Y/%m/%d %H:%M:%S").unwrap();
        let date_time_local: DateTime<Local> = Local.from_local_datetime(&dt).unwrap();
        assert_eq!(date_to_string(&date_time_local), "12/31(土)".to_string());

        let dt: NaiveDateTime =
            NaiveDateTime::parse_from_str("2023/01/01 00:00:00", "%Y/%m/%d %H:%M:%S").unwrap();
        let date_time_local: DateTime<Local> = Local.from_local_datetime(&dt).unwrap();
        assert_eq!(date_to_string(&date_time_local), "01/01(日)".to_string());
    }

    #[test]
    fn test_date_to_yyyymmddhhmmss() {
        let dt: NaiveDateTime =
            NaiveDateTime::parse_from_str("2022/02/22 22:22:22", "%Y/%m/%d %H:%M:%S").unwrap();
        let date_time_local: DateTime<Local> = Local.from_local_datetime(&dt).unwrap();
        assert_eq!(
            date_to_yyyymmddhhmmss(&date_time_local),
            "20220222222222".to_string()
        );

        let dt: NaiveDateTime =
            NaiveDateTime::parse_from_str("2022/12/31 23:59:59", "%Y/%m/%d %H:%M:%S").unwrap();
        let date_time_local: DateTime<Local> = Local.from_local_datetime(&dt).unwrap();
        assert_eq!(
            date_to_yyyymmddhhmmss(&date_time_local),
            "20221231235959".to_string()
        );

        let dt: NaiveDateTime =
            NaiveDateTime::parse_from_str("2023/01/01 00:00:00", "%Y/%m/%d %H:%M:%S").unwrap();
        let date_time_local: DateTime<Local> = Local.from_local_datetime(&dt).unwrap();
        assert_eq!(
            date_to_yyyymmddhhmmss(&date_time_local),
            "20230101000000".to_string()
        );
    }
}
