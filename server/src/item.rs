use crate::model::item::{
    InputNewItem, ItemEditResponse, ItemListResponse, ItemNewResponse, ItemWithMakerAndWorker,
    ItemsPutRequest, TitleFiltered, TitleWithItems, YearMonthList,
};
use crate::setting::AppState;
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
    let year_month_not_null_list = YearMonthList::find_by_statement(
        Statement::from_sql_and_values(DbBackend::Postgres, year_month_sql, vec![]),
    )
    .all(conn)
    .await
    .expect("could not find items.");

    let mut year_month_list = vec![YearMonthList {
        yyyymm: "発売日未定".to_string(),
        year: "".to_string(),
        month: "".to_string(),
    }];
    year_month_list.extend(year_month_not_null_list);

    // year_month_listの長さ分だけ、対応する日付のitemを取得する
    let mut title_list_group_by_year_month = Vec::new();
    let title_sql_select = "
        SELECT
            id,
            name,
            release_date,
            delivery_date,
            list_submission_date,
            reservation_start_date,
            reservation_deadline,
            order_date_to_maker,
            updated_at,
            project_type,
            catalog_status,
            announcement_status,
            remarks
        FROM
            title
    ";
    for year_month in year_month_list {
        let title_sql = if year_month.yyyymm == "発売日未定" {
            format!(
                "{}{}",
                title_sql_select,
                "
                    WHERE
                        deleted = FALSE
                    AND
                        release_date is NULL
                    ORDER BY
                        release_date ASC,
                        CASE project_type WHEN 'S案件' THEN 1 
                                          WHEN 'Y案件' THEN 2 
                                          WHEN 'デフォルト' THEN 3 
                                          WHEN '再販' THEN 4 
                                          ELSE 5 END ASC
                        ;
                "
            )
        } else {
            let year = year_month.year.clone();
            let month = year_month.month.clone();
            let end_day = last_day_of_month(year.clone(), month.clone());
            let year_month_start = format!("{}-{}-01 00:00:00", year.clone(), month.clone());
            let year_month_end = format!("{}-{}-{} 23:59:59", year.clone(), month.clone(), end_day);
            format!(
                "{}
                    WHERE
                        deleted = FALSE
                    AND
                        release_date
                        BETWEEN
                            '{}'
                        AND
                            '{}'
                    ORDER BY
                        release_date ASC,
                        CASE project_type WHEN 'S案件' THEN 1 
                                          WHEN 'Y案件' THEN 2 
                                          WHEN 'デフォルト' THEN 3 
                                          WHEN '再販' THEN 4 
                                          ELSE 5 END ASC
                        ;
                ",
                title_sql_select, year_month_start, year_month_end
            )
        };

        let titles = TitleFiltered::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            &title_sql,
            vec![],
        ))
        .all(conn)
        .await
        .expect("could not find titles.");

        let mut title_with_items = vec![];
        let mut item_count = 0;
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
                    pic_design.name AS pic_design,
                    item.maker_id,
                    maker.code_name AS maker_code,
                    item.retail_price,
                    item.resubmission,
                    item.double_check_person_id,
                    item.line,
                    double_check_person.name AS double_check_person,
                    item.rough_coordinator_id,
                    rough_coordinator.name AS rough_coordinator,
                    item.rough_check_person_id,
                    rough_check_person.name AS rough_check_person,
                    item.line_art_coordinator_id,
                    line_art_coordinator.name AS line_art_coordinator,
                    item.line_art_check_person_id,
                    line_art_check_person.name AS line_art_check_person,
                    item.coloring_coordinator_id,
                    coloring_coordinator.name AS coloring_coordinator,
                    item.coloring_check_person_id,
                    coloring_check_person.name AS coloring_check_person,
                    item.design_coordinator_id,
                    design_coordinator.name AS design_coordinator,
                    item.design_check_person_id,
                    design_check_person.name AS design_check_person,
                    item.submission_data_coordinator_id,
                    submission_data_coordinator.name AS submission_data_coordinator,
                    item.submission_data_check_person_id,
                    submission_data_check_person.name AS submission_data_check_person,
                    item.announcement_materials_coordinator_id,
                    announcement_materials_coordinator.name AS announcement_materials_coordinator,
                    item.announcement_materials_check_person_id,
                    announcement_materials_check_person.name AS announcement_materials_check_person,
                    item.jan_coordinator_id,
                    jan_coordinator.name AS jan_coordinator,
                    item.jan_check_person_id,
                    jan_check_person.name AS jan_check_person
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
                LEFT JOIN
                    worker AS rough_coordinator ON item.rough_coordinator_id = rough_coordinator.id
                LEFT JOIN
                    worker AS rough_check_person ON item.rough_check_person_id = rough_check_person.id
                LEFT JOIN
                    worker AS line_art_coordinator ON item.line_art_coordinator_id = line_art_coordinator.id
                LEFT JOIN
                    worker AS line_art_check_person ON item.line_art_check_person_id = line_art_check_person.id
                LEFT JOIN
                    worker AS coloring_coordinator ON item.coloring_coordinator_id = coloring_coordinator.id
                LEFT JOIN
                    worker AS coloring_check_person ON item.coloring_check_person_id = coloring_check_person.id
                LEFT JOIN
                    worker AS design_coordinator ON item.design_coordinator_id = design_coordinator.id
                LEFT JOIN
                    worker AS design_check_person ON item.design_check_person_id = design_check_person.id
                LEFT JOIN
                    worker AS submission_data_coordinator ON item.submission_data_coordinator_id = submission_data_coordinator.id
                LEFT JOIN
                    worker AS submission_data_check_person ON item.submission_data_check_person_id = submission_data_check_person.id
                LEFT JOIN
                    worker AS announcement_materials_coordinator ON item.announcement_materials_coordinator_id = announcement_materials_coordinator.id
                LEFT JOIN
                    worker AS announcement_materials_check_person ON item.announcement_materials_check_person_id = announcement_materials_check_person.id
                LEFT JOIN
                    worker AS jan_coordinator ON item.jan_coordinator_id = jan_coordinator.id
                LEFT JOIN
                    worker AS jan_check_person ON item.jan_check_person_id = jan_check_person.id
                WHERE
                    title_id = '{}'
                AND
                    item.deleted = FALSE
                ORDER BY
                    item.product_code ASC NULLS FIRST;
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
            let release_date: Option<DateTimeWithTimeZone> = title
                .release_date
                .map(|release_date| release_date.with_timezone(&FixedOffset::east(9 * 3600)));
            let delivery_date = title
                .delivery_date
                .map(|delivery_date| delivery_date.with_timezone(&FixedOffset::east(9 * 3600)));
            let list_submission_date = title.list_submission_date.map(|list_submission_date| {
                list_submission_date.with_timezone(&FixedOffset::east(9 * 3600))
            });
            let reservation_start_date =
                title.reservation_start_date.map(|reservation_start_date| {
                    reservation_start_date.with_timezone(&FixedOffset::east(9 * 3600))
                });
            let reservation_deadline = title.reservation_deadline.map(|reservation_deadline| {
                reservation_deadline.with_timezone(&FixedOffset::east(9 * 3600))
            });
            let order_date_to_maker = title.order_date_to_maker.map(|order_date_to_maker| {
                order_date_to_maker.with_timezone(&FixedOffset::east(9 * 3600))
            });
            let updated_at = title.updated_at.with_timezone(&FixedOffset::east(9 * 3600));
            item_count += items.len();
            title_with_items.push(TitleWithItems {
                id: title.id,
                name: title.name,
                release_date,
                delivery_date,
                list_submission_date,
                reservation_start_date,
                reservation_deadline,
                order_date_to_maker,
                updated_at,
                project_type: title.project_type,
                catalog_status: title.catalog_status,
                announcement_status: title.announcement_status,
                remarks: title.remarks,
                items,
            });
        }
        title_list_group_by_year_month.push(ItemListResponse {
            yyyymm: year_month.yyyymm,
            year: year_month.year,
            month: year_month.month,
            title_count: title_with_items.len(),
            item_count,
            title_list: title_with_items,
        });
    }

    Ok(HttpResponse::Ok().json(title_list_group_by_year_month))
}

#[get("/api/item_new")]
async fn api_item_new(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let workers = Worker::find()
        .order_by_asc(worker::Column::Name)
        .filter(worker::Column::Deleted.eq(false))
        .all(conn)
        .await
        .expect("could not find workers.");

    let makers = Maker::find()
        .filter(maker::Column::Deleted.eq(false))
        .order_by_asc(maker::Column::CodeName)
        .all(conn)
        .await
        .expect("could not find makers.");

    Ok(HttpResponse::Ok().json(ItemNewResponse { makers, workers }))
}

#[post("/api/item_new")]
async fn api_create_items(
    data: web::Data<AppState>,
    post_data: web::Json<InputNewItem>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let release_date: Option<DateTimeWithTimeZone> = post_data
        .release_date
        .map(|release_date| release_date.with_timezone(&FixedOffset::east(9 * 3600)));
    let delivery_date = post_data
        .delivery_date
        .map(|delivery_date| delivery_date.with_timezone(&FixedOffset::east(9 * 3600)));
    let list_submission_date = post_data.list_submission_date.map(|list_submission_date| {
        list_submission_date.with_timezone(&FixedOffset::east(9 * 3600))
    });
    let reservation_start_date = post_data
        .reservation_start_date
        .map(|reservation_start_date| {
            reservation_start_date.with_timezone(&FixedOffset::east(9 * 3600))
        });
    let reservation_deadline = post_data.reservation_deadline.map(|reservation_deadline| {
        reservation_deadline.with_timezone(&FixedOffset::east(9 * 3600))
    });
    let order_date_to_maker = post_data
        .order_date_to_maker
        .map(|order_date_to_maker| order_date_to_maker.with_timezone(&FixedOffset::east(9 * 3600)));

    // title登録
    let title = title::ActiveModel {
        id: Set(post_data.title_id),
        name: Set(post_data.title_name.clone()),
        release_date: Set(release_date),
        delivery_date: Set(delivery_date),
        list_submission_date: Set(list_submission_date),
        reservation_start_date: Set(reservation_start_date),
        reservation_deadline: Set(reservation_deadline),
        order_date_to_maker: Set(order_date_to_maker),
        updated_at: Set(Utc::now().into()),
        project_type: Set(post_data.project_type.clone()),
        catalog_status: Set(post_data.catalog_status.clone()),
        announcement_status: Set(post_data.announcement_status.clone()),
        remarks: Set(post_data.remarks.clone()),
        deleted: Set(false),
    }
    .insert(conn)
    .await
    .expect("could not insert title.");

    // item登録
    let title_id = title.id;
    for item in post_data.items.iter() {
        item::ActiveModel {
            id: Set(item.id),
            title_id: Set(title_id),
            name: Set(item.name.clone()),
            product_code: Set(item.product_code.clone()),
            sku: Set(item.sku),
            illust_status: Set(item.illust_status.clone()),
            pic_illust_id: Set(item.pic_illust_id),
            design_status: Set(item.design_status.clone()),
            pic_design_id: Set(item.pic_design_id),
            maker_id: Set(item.maker_id),
            retail_price: Set(item.retail_price),
            double_check_person_id: Set(item.double_check_person_id),
            rough_coordinator_id: Set(item.rough_coordinator_id),
            rough_check_person_id: Set(item.rough_check_person_id),
            line_art_coordinator_id: Set(item.line_art_coordinator_id),
            line_art_check_person_id: Set(item.line_art_check_person_id),
            coloring_coordinator_id: Set(item.coloring_coordinator_id),
            coloring_check_person_id: Set(item.coloring_check_person_id),
            design_coordinator_id: Set(item.design_coordinator_id),
            design_check_person_id: Set(item.design_check_person_id),
            submission_data_coordinator_id: Set(item.submission_data_coordinator_id),
            submission_data_check_person_id: Set(item.submission_data_check_person_id),
            announcement_materials_coordinator_id: Set(item.announcement_materials_coordinator_id),
            announcement_materials_check_person_id: Set(item.announcement_materials_check_person_id),
            jan_coordinator_id: Set(item.jan_coordinator_id),
            jan_check_person_id: Set(item.jan_check_person_id),
            deleted: Set(false),
            resubmission: Set(item.resubmission),
            line: Set(item.line.clone()),
        }
        .insert(conn)
        .await
        .expect("could not insert item");
    }

    Ok(HttpResponse::Created().finish())
}

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
        .order_by_asc(item::Column::ProductCode)
        .filter(item::Column::TitleId.eq(title_id.to_owned()))
        .filter(item::Column::Deleted.eq(false))
        .all(conn)
        .await
        .expect("could not find items by title_id.");

    let workers = Worker::find()
        .order_by_asc(worker::Column::Name)
        .filter(worker::Column::Deleted.eq(false))
        .all(conn)
        .await
        .expect("could not find workers.");

    let makers = Maker::find()
        .filter(maker::Column::Deleted.eq(false))
        .order_by_asc(maker::Column::CodeName)
        .all(conn)
        .await
        .expect("could not find makers.");

    let release_date: Option<DateTimeWithTimeZone> = title
        .release_date
        .map(|release_date| release_date.with_timezone(&FixedOffset::east(9 * 3600)));
    let delivery_date = title
        .delivery_date
        .map(|delivery_date| delivery_date.with_timezone(&FixedOffset::east(9 * 3600)));
    let list_submission_date = title.list_submission_date.map(|list_submission_date| {
        list_submission_date.with_timezone(&FixedOffset::east(9 * 3600))
    });
    let reservation_start_date = title.reservation_start_date.map(|reservation_start_date| {
        reservation_start_date.with_timezone(&FixedOffset::east(9 * 3600))
    });
    let reservation_deadline = title.reservation_deadline.map(|reservation_deadline| {
        reservation_deadline.with_timezone(&FixedOffset::east(9 * 3600))
    });
    let order_date_to_maker = title
        .order_date_to_maker
        .map(|order_date_to_maker| order_date_to_maker.with_timezone(&FixedOffset::east(9 * 3600)));
    let updated_at = title.updated_at.with_timezone(&FixedOffset::east(9 * 3600));

    Ok(HttpResponse::Ok().json(ItemEditResponse {
        items,
        workers,
        makers,
        release_date,
        delivery_date,
        list_submission_date,
        reservation_start_date,
        reservation_deadline,
        order_date_to_maker,
        updated_at,
        title: title_name,
        project_type,
        catalog_status,
        announcement_status,
        remarks,
    }))
}

#[put("/api/item")]
async fn api_update_items(
    data: web::Data<AppState>,
    post_data: web::Json<ItemsPutRequest>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let put_data = post_data.into_inner();
    let title_id = put_data.title_id;

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
        delivery_date: match put_data.delivery_date {
            Some(delivery_date) => Set(Some(utc_date_time_to_jst(&delivery_date))),
            None => Set(None),
        },
        list_submission_date: match put_data.list_submission_date {
            Some(list_submission_date) => Set(Some(utc_date_time_to_jst(&list_submission_date))),
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
        updated_at: Set(Utc::now().into()),
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
        let item_id = item.id;
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
                    resubmission: Set(item.resubmission),
                    double_check_person_id: Set(item.double_check_person_id.to_owned()),
                    line: Set(item.line.to_owned()),
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
                    resubmission: Set(item.resubmission),
                    double_check_person_id: Set(item.double_check_person_id.to_owned()),
                    line: Set(item.line.to_owned()),
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

    // idに一致するitemのdeletedカラムをtrueにする
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

#[delete("/api/delete_title/{title_id}")]
async fn api_delete_title(
    data: web::Data<AppState>,
    title_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    // idに一致するtitleのdeletedカラムをtrueにする
    title::ActiveModel {
        id: Set(title_id.into_inner()),
        deleted: Set(true),
        ..Default::default()
    }
    .update(conn)
    .await
    .expect("could not delete item.");

    Ok(HttpResponse::Ok().finish())
}

fn _date_to_string(date_time: &DateTime<Utc>) -> String {
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

    use crate::item::_date_to_string;
    use crate::item::last_day_of_month;

    #[test]
    fn test_date_to_string() {
        let date_time = Utc.ymd(2022, 2, 21).and_hms(16, 0, 0);
        assert_eq!(_date_to_string(&date_time), "02/22(火)".to_string());

        let date_time = Utc.ymd(2022, 12, 31).and_hms(11, 0, 0);
        assert_eq!(_date_to_string(&date_time), "12/31(土)".to_string());

        let date_time = Utc.ymd(2022, 12, 31).and_hms(16, 0, 0);
        assert_eq!(_date_to_string(&date_time), "01/01(日)".to_string());

        let date_time = Utc.ymd(2022, 1, 1).and_hms(0, 0, 0);
        assert_eq!(_date_to_string(&date_time), "01/01(土)".to_string());
    }

    #[test]
    fn test_last_day_of_month() {
        assert_eq!(
            last_day_of_month("2021".to_string(), "1".to_string()),
            "31".to_string()
        );
        assert_eq!(
            last_day_of_month("2021".to_string(), "2".to_string()),
            "28".to_string()
        );
        assert_eq!(
            last_day_of_month("2021".to_string(), "3".to_string()),
            "31".to_string()
        );
        assert_eq!(
            last_day_of_month("2021".to_string(), "4".to_string()),
            "30".to_string()
        );
        assert_eq!(
            last_day_of_month("2021".to_string(), "5".to_string()),
            "31".to_string()
        );
        assert_eq!(
            last_day_of_month("2021".to_string(), "6".to_string()),
            "30".to_string()
        );
        assert_eq!(
            last_day_of_month("2021".to_string(), "7".to_string()),
            "31".to_string()
        );
        assert_eq!(
            last_day_of_month("2021".to_string(), "8".to_string()),
            "31".to_string()
        );
        assert_eq!(
            last_day_of_month("2021".to_string(), "9".to_string()),
            "30".to_string()
        );
        assert_eq!(
            last_day_of_month("2021".to_string(), "10".to_string()),
            "31".to_string()
        );
        assert_eq!(
            last_day_of_month("2021".to_string(), "11".to_string()),
            "30".to_string()
        );
        assert_eq!(
            last_day_of_month("2021".to_string(), "12".to_string()),
            "31".to_string()
        );
        // うるう年
        assert_eq!(
            last_day_of_month("2020".to_string(), "1".to_string()),
            "31".to_string()
        );
        assert_eq!(
            last_day_of_month("2020".to_string(), "2".to_string()),
            "29".to_string()
        );
        assert_eq!(
            last_day_of_month("2020".to_string(), "3".to_string()),
            "31".to_string()
        );
        assert_eq!(
            last_day_of_month("2020".to_string(), "4".to_string()),
            "30".to_string()
        );
        assert_eq!(
            last_day_of_month("2020".to_string(), "5".to_string()),
            "31".to_string()
        );
        assert_eq!(
            last_day_of_month("2020".to_string(), "6".to_string()),
            "30".to_string()
        );
        assert_eq!(
            last_day_of_month("2020".to_string(), "7".to_string()),
            "31".to_string()
        );
        assert_eq!(
            last_day_of_month("2020".to_string(), "8".to_string()),
            "31".to_string()
        );
        assert_eq!(
            last_day_of_month("2020".to_string(), "9".to_string()),
            "30".to_string()
        );
        assert_eq!(
            last_day_of_month("2020".to_string(), "10".to_string()),
            "31".to_string()
        );
        assert_eq!(
            last_day_of_month("2020".to_string(), "11".to_string()),
            "30".to_string()
        );
        assert_eq!(
            last_day_of_month("2020".to_string(), "12".to_string()),
            "31".to_string()
        );
    }
}
