use crate::setting::AppState;
use actix_web::{error, get, post, web, Error, HttpResponse, Result};
use chrono::Local;
use entity::worker;
use entity::worker::Entity as Worker;
use sea_orm::{entity::*, query::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct InputNewWorker {
    name: String,
    description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct UpdateWorker {
    name: String,
    description: Option<String>,
}

#[get("/worker")]
async fn worker_list(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let conn = &data.conn;
    let datas = Worker::find()
        .order_by_asc(worker::Column::Id)
        .filter(worker::Column::Deleted.eq(false))
        .all(conn)
        .await
        .expect("could not retrieve datas");
    let mut ctx = tera::Context::new();
    let h1 = "関係者";
    let path = "worker";

    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    struct ViewData {
        id: i32,
        name: String,
        description: Option<String>,
        created_at: String,
        updated_at: String,
    }

    let view_datas = datas
        .iter()
        .map(|worker| ViewData {
            id: worker.id,
            name: worker.name.clone(),
            description: worker.description.clone(),
            created_at: worker
                .created_at
                .format("%Y年%m月%d日 <br> %H時%M分%S秒")
                .to_string(),
            updated_at: worker
                .updated_at
                .format("%Y年%m月%d日 <br> %H時%M分%S秒")
                .to_string(),
        })
        .collect::<Vec<ViewData>>();

    ctx.insert("datas", &view_datas);
    ctx.insert("h1", &h1);
    ctx.insert("path", &path);

    let body = template
        .render("worker/worker_list.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/new_worker")]
async fn new_worker(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let ctx = tera::Context::new();
    let body = template
        .render("worker/new_worker.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/new_worker")]
async fn create_worker(
    data: web::Data<AppState>,
    post_form: web::Form<InputNewWorker>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = post_form.into_inner();

    let date = Local::now();

    worker::ActiveModel {
        name: Set(form.name.to_owned()),
        description: Set(form.description.to_owned()),
        created_at: Set(date),
        updated_at: Set(date),
        ..Default::default()
    }
    .insert(conn)
    .await
    .expect("could not insert worker");

    Ok(HttpResponse::Found()
        .append_header(("location", "/new_worker"))
        .finish())
}

#[get("/worker/{id}")]
async fn edit_worker(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let template = &data.templates;

    let worker: worker::Model = Worker::find_by_id(id.into_inner())
        .one(conn)
        .await
        .expect("could not find worker")
        .unwrap();

    let mut ctx = tera::Context::new();
    let path = "worker";
    ctx.insert("worker", &worker);
    ctx.insert("path", &path);

    let body = template
        .render("worker/edit_worker.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/worker/{id}")]
async fn update_worker(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    post_form: web::Form<UpdateWorker>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = post_form.into_inner();

    let date = Local::now();

    worker::ActiveModel {
        id: Set(*id),
        name: Set(form.name.to_owned()),
        description: Set(form.description.to_owned()),
        updated_at: Set(date),
        ..Default::default()
    }
    .save(conn)
    .await
    .expect("could not edit worker");

    Ok(HttpResponse::Found()
        .append_header(("location", "/worker"))
        .finish())
}

#[post("/delete_worker/{id}")]
async fn delete_worker(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    worker::ActiveModel {
        id: Set(*id),
        deleted: Set(true),
        ..Default::default()
    }
    .save(conn)
    .await
    .expect("could not delete worker");

    Ok(HttpResponse::Found()
        .append_header(("location", "/worker"))
        .finish())
}