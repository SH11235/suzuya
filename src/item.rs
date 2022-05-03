use std::fmt::Debug;

use crate::setting::{AppState, Params, DEFAULT_ITEMS_PER_PAGE, ITME_INPUT_NUM};
use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use chrono::{DateTime, Local};
use entity::item;
use entity::item::Entity as Item;
use sea_orm::{entity::*, prelude::DateTimeLocal, query::*};
use sea_orm::{DbBackend, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct InputNewItem {
    title: String,
    name_list: String,
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
        title: String,
        name: String,
        release_date: Option<DateTimeLocal>,
        product_code: Option<String>,
        reservation_start_date: Option<DateTimeLocal>,
        reservation_deadline: Option<DateTimeLocal>,
        order_date: Option<DateTimeLocal>,
        sku: Option<i32>,
        illust_status: String,
        design_status: String,
        last_updated: DateTimeLocal,
        retail_price: Option<i32>,
        catalog_status: String,
        announcement_status: String,
        remarks: Option<String>,
        // 外部キー
        maker_code: Option<String>,
        pic: Option<String>,
        double_check_person: Option<String>,
    }
    let paginator = SelectResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
                    "items"."id",
                    "items"."title",
                    "items"."name",
                    "items"."product_code",
                    "items"."release_date",
                    "items"."reservation_start_date",
                    "items"."reservation_deadline",
                    "items"."order_date",
                    "items"."sku",
                    "items"."illust_status",
                    "items"."design_status",
                    "items"."last_updated",
                    "items"."retail_price",
                    "items"."catalog_status",
                    "items"."announcement_status",
                    "items"."remarks",
                    "makers"."code_name" AS "maker_code",
                    "pics"."name" AS "pic",
                    "users"."name" AS "double_check_person"
                FROM
                    "items"
                    LEFT JOIN "makers" ON "items"."maker_id" = "makers"."id"
                    LEFT JOIN "users" AS "pics" ON "items"."pic_id" = "pics"."id"
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

    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    struct ViewData {
        id: i32,
        title: String,
        name: String,
        release_date: Option<String>,
        product_code: Option<String>,
        reservation_start_date: Option<String>, // 予約開始日(BtoBおよびBtoC)
        reservation_deadline: Option<String>,   // 予約締切日
        order_date: Option<String>,             // メーカーへの発注日
        sku: Option<i32>,                       // 種類数
        illust_status: String,
        design_status: String,
        last_updated: String,      // 最終更新日（ステータス変更時）
        retail_price: Option<i32>, // 上代
        catalog_status: String,
        announcement_status: String,
        remarks: Option<String>, // 備考
        // 外部キー
        maker_code: Option<String>,          // from maker
        pic: Option<String>,                 // from user 「担当者」person in charge
        double_check_person: Option<String>, // from user 社員名
    }

    let view_datas = datas
        .iter()
        .map(|item| {
            ViewData {
                id: item.id,
                title: item.title.clone(),
                name: item.name.clone(),
                product_code: item.product_code.clone(),
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
                sku: item.sku,
                illust_status: item.illust_status.clone(),
                design_status: item.design_status.clone(),
                last_updated: date_to_string(&item.last_updated),
                retail_price: item.retail_price,
                catalog_status: item.catalog_status.clone(),
                announcement_status: item.announcement_status.clone(),
                remarks: item.remarks.clone(),
                // 外部キー
                maker_code: item.maker_code.clone(),
                pic: item.pic.clone(),
                double_check_person: item.double_check_person.clone(),
            }
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
async fn create_item(
    data: web::Data<AppState>,
    post_form: web::Form<InputNewItem>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let count = Item::find()
        .column(item::Column::Id)
        .all(conn)
        .await
        .unwrap()
        .len() as i32;
    let mut id = count + 1;
    let form = post_form.into_inner();
    let name_list: Vec<&str> = form.name_list.split(',').collect();

    println!("{:?}", name_list);
    let last_updated = Local::now();
    for item_name in name_list.iter() {
        item::ActiveModel {
            id: Set(id),
            title: Set(form.title.to_owned()),
            name: Set(item_name.to_string()),
            last_updated: Set(last_updated),
            ..Default::default()
        }
        .insert(conn)
        .await
        .expect("could not insert item");
        id = id + 1;
    }

    Ok(HttpResponse::Found()
        .append_header(("location", "/new_item"))
        .finish())
}

fn date_to_string(date_time: &DateTime<Local>) -> String {
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