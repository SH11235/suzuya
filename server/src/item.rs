use crate::model::item::{
    ItemEditResponse, ItemListResponse, ItemWithMakerAndWorker, ItemsPutRequest, TitleFiltered,
    TitleWithItems, YearMonthList,
};
use crate::setting::{
    announce_status_list, catalog_status_list, design_status_list, illust_status_list,
    project_type_list, AppState,
};
use actix_web::{delete, get, post, put, web, Error, HttpResponse, Result};
use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, Utc};
use entity::item::Entity as Item;
use entity::maker::Entity as Maker;
use entity::title::Entity as Title;
use entity::worker::Entity as Worker;
use entity::{item, maker, title, worker};
use sea_orm::prelude::{DateTimeWithTimeZone, Uuid};
use sea_orm::DbBackend;
use sea_orm::{entity::*, query::*};

#[get("/api/item_list")]
async fn api_item_list(
    data: web::Data<AppState>,
    // query: web::Query<ItemListQuery>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let year_month_sql = "
        SELECT
            to_char(release_date, 'YYYY/MM') as yyyymm,
            to_char(release_date, 'YYYY') as year,
            to_char(release_date, 'MM') as month
        FROM title
        WHERE release_date is not NULL
        AND deleted = FALSE 
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

    // year_month_listの長さ分だけ、対応する日付のitemを取得する
    let mut title_with_items = Vec::new();
    for year_month in year_month_list.clone() {
        let year = year_month.year;
        let month = year_month.month;
        let end_day = last_day_of_month(year.clone(), month.clone());
        let year_month_start = format!("{}-{}-01 00:00:00", year.clone(), month.clone());
        let year_month_end = format!("{}-{}-{} 23:59:59", year.clone(), month.clone(), end_day);
        let title_sql = format!(
            "SELECT
                id,
                name,
                release_date,
                reservation_start_date,
                reservation_deadline,
                order_date_to_maker,
                project_type,
                catalog_status,
                announcement_status,
                remarks
            FROM
                title
            WHERE
                deleted = FALSE
            AND
                release_date
                BETWEEN
                    '{}'
                AND
                    '{}'
            ORDER BY
                reservation_start_date ASC NULLS FIRST;
            ",
            year_month_start, year_month_end
        );
        let titles = TitleFiltered::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            &title_sql,
            vec![],
        ))
        .all(conn)
        .await
        .expect("could not find titles.");

        for title in titles.clone() {
            let title_id = title.id;
            let item_sql = format!(
                "SELECT
                    item.id,
                    item.name,
                    item.product_code,
                    item.sku,
                    item.illust_status,
                    item.pic_illust_id,
                    pic_illust.name AS pic_illust,
                    item.design_status,
                    item.pic_design_id,
                    pic_illust.name AS pic_design,
                    item.maker_id,
                    maker.code_name AS maker_code,
                    item.retail_price,
                    item.double_check_person_id,
                    double_check_person.name AS double_check_person
                FROM
                    item
                LEFT JOIN 
                    maker ON item.maker_id = maker.id
                LEFT JOIN 
                    worker AS pic_illust ON item.pic_illust_id = pic_illust.id
                LEFT JOIN
                    worker AS pic_design ON item.pic_design_id = pic_design.id
                LEFT JOIN
                    worker AS double_check_person ON item.double_check_person_id = double_check_person.id
                WHERE
                    title_id = '{}'
                AND
                    item.deleted = FALSE
                ORDER BY
                    id ASC NULLS FIRST;
                ",
                title_id
            );
            let items = ItemWithMakerAndWorker::find_by_statement(Statement::from_sql_and_values(
                DbBackend::Postgres,
                &item_sql,
                vec![],
            ))
            .all(conn)
            .await
            .expect("could not find items.");
            title_with_items.push(TitleWithItems {
                id: title.id,
                name: title.name,
                release_date: title.release_date,
                reservation_start_date: title.reservation_start_date,
                reservation_deadline: title.reservation_deadline,
                order_date_to_maker: title.order_date_to_maker,
                project_type: title.project_type,
                catalog_status: title.catalog_status,
                announcement_status: title.announcement_status,
                remarks: title.remarks,
                items: items,
            });
        }
    }

    let response = year_month_list.iter().map(|year_month| {
        ItemListResponse {
            yyyymm: year_month.yyyymm.clone(),
            year: year_month.year.clone(),
            month:  year_month.month.clone(),
            title_list: title_with_items.clone(),
        }
    })
    .collect::<Vec<ItemListResponse>>();

    Ok(HttpResponse::Ok().json(response))
}

// #[get("/new_item")]
// async fn new_item(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
//     let template = &data.templates;
//     let mut ctx = tera::Context::new();
//     let h4 = "アイテム登録";
//     let path = "item";
//     let input_id_list: Vec<i32> = (1..ITME_INPUT_NUM + 1).collect();
//     let spacer = ",";

//     ctx.insert("h4", &h4);
//     ctx.insert("path", &path);
//     ctx.insert("input_id_list", &input_id_list);
//     ctx.insert("spacer", &spacer);
//     let body = template
//         .render("item/new_item.html.tera", &ctx)
//         .map_err(|_| error::ErrorInternalServerError("Template error"))?;
//     Ok(HttpResponse::Ok().content_type("text/html").body(body))
// }

// #[post("/new_item")]
// async fn create_items(
//     data: web::Data<AppState>,
//     post_data: web::Json<InputNewItem>,
// ) -> Result<HttpResponse, Error> {
//     let conn = &data.conn;

//     let form = post_data.into_inner();
//     let name_list = form.name_list;

//     // title登録
//     let title = title::ActiveModel {
//         name: Set(form.title),
//         deleted: Set(false),
//         ..Default::default()
//     }
//     .insert(conn)
//     .await
//     .expect("could not insert title.");

//     // item登録
//     let title_id = title.id;
//     for item_name in name_list.iter() {
//         item::ActiveModel {
//             title_id: Set(title_id),
//             name: Set(item_name.to_string()),
//             ..Default::default()
//         }
//         .insert(conn)
//         .await
//         .expect("could not insert item");
//     }

//     Ok(HttpResponse::Found()
//         .append_header(("location", "/new_item"))
//         .finish())
// }

#[get("/api/item/{title_id}")]
async fn api_item_edit_page(
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
        .filter(item::Column::Deleted.eq(false))
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

    Ok(HttpResponse::Ok().json(ItemEditResponse {
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
async fn api_update_items(
    data: web::Data<AppState>,
    post_data: web::Json<ItemsPutRequest>,
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
        let item_id = item.id.clone();
        // item_id存在確認
        let item_id_exist = Item::find_by_id(item_id)
            .one(conn)
            .await
            .expect("could not find item.");

        match item_id_exist {
            Some(_) => {
                // idあり→UPDATE
                item::ActiveModel {
                    id: Set(item_id.to_owned()),
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
                    id: Set(item_id.to_owned()),
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

#[delete("/api/delete_item/{id}")]
async fn api_delete_item(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    // idに一致するitemをのdeletedカラムをtrueにする
    item::ActiveModel {
        id: Set(id.into_inner()),
        deleted: Set(true),
        ..Default::default()
    }
    .update(conn)
    .await
    .expect("could not delete item.");

    Ok(HttpResponse::Ok().finish())
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

// 月の最終日を返す関数
fn last_day_of_month(year: String, month: String) -> String {
    let year: i32 = year.parse().unwrap();
    let month: u32 = month.parse().unwrap();
    // n月1日から1日戻った日付を取得するので、monthを調整する
    let month = if month == 12 { 1 } else { month + 1 };
    let last_day = NaiveDate::from_ymd(year, month, 1)
        .with_month(month)
        .unwrap()
        .pred()
        .day();
    last_day.to_string()
}
#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::item::date_to_string;
    use crate::item::last_day_of_month;

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

    #[test]
    fn test_last_day_of_month() {
        assert_eq!(last_day_of_month("2021".to_string(), "1".to_string()), "31".to_string());
        assert_eq!(last_day_of_month("2021".to_string(), "2".to_string()), "28".to_string());
        assert_eq!(last_day_of_month("2021".to_string(), "3".to_string()), "31".to_string());
        assert_eq!(last_day_of_month("2021".to_string(), "4".to_string()), "30".to_string());
        assert_eq!(last_day_of_month("2021".to_string(), "5".to_string()), "31".to_string());
        assert_eq!(last_day_of_month("2021".to_string(), "6".to_string()), "30".to_string());
        assert_eq!(last_day_of_month("2021".to_string(), "7".to_string()), "31".to_string());
        assert_eq!(last_day_of_month("2021".to_string(), "8".to_string()), "31".to_string());
        assert_eq!(last_day_of_month("2021".to_string(), "9".to_string()), "30".to_string());
        assert_eq!(last_day_of_month("2021".to_string(), "10".to_string()), "31".to_string());
        assert_eq!(last_day_of_month("2021".to_string(), "11".to_string()), "30".to_string());
        assert_eq!(last_day_of_month("2021".to_string(), "12".to_string()), "31".to_string());
        // うるう年
        assert_eq!(last_day_of_month("2020".to_string(), "1".to_string()), "31".to_string());
        assert_eq!(last_day_of_month("2020".to_string(), "2".to_string()), "29".to_string());
        assert_eq!(last_day_of_month("2020".to_string(), "3".to_string()), "31".to_string());
        assert_eq!(last_day_of_month("2020".to_string(), "4".to_string()), "30".to_string());
        assert_eq!(last_day_of_month("2020".to_string(), "5".to_string()), "31".to_string());
        assert_eq!(last_day_of_month("2020".to_string(), "6".to_string()), "30".to_string());
        assert_eq!(last_day_of_month("2020".to_string(), "7".to_string()), "31".to_string());
        assert_eq!(last_day_of_month("2020".to_string(), "8".to_string()), "31".to_string());
        assert_eq!(last_day_of_month("2020".to_string(), "9".to_string()), "30".to_string());
        assert_eq!(last_day_of_month("2020".to_string(), "10".to_string()), "31".to_string());
        assert_eq!(last_day_of_month("2020".to_string(), "11".to_string()), "30".to_string());
        assert_eq!(last_day_of_month("2020".to_string(), "12".to_string()), "31".to_string());
    }
}
