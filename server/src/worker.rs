use crate::setting::AppState;
use actix_web::{error, get, post, put, web, Error, HttpResponse, Result, delete};
use entity::worker;
use entity::worker::Entity as Worker;
use sea_orm::{entity::*, prelude::Uuid, query::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct InputNewWorker {
    name: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct UpdateWorker {
    name: String,
}

#[get("/api/worker_list")]
async fn api_worker_list(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let datas = Worker::find()
        .order_by_asc(worker::Column::Name)
        .filter(worker::Column::Deleted.eq(false))
        .all(conn)
        .await
        .expect("could not retrieve datas");

    Ok(HttpResponse::Ok().json(datas))
}

#[post("/api/new_worker")]
async fn api_create_worker(
    data: web::Data<AppState>,
    request_body: web::Json<InputNewWorker>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let uuid = Uuid::new_v4();

    worker::ActiveModel {
        id: Set(uuid.clone()),
        name: Set(request_body.name.to_owned()),
        deleted: Set(false),
    }
    .insert(conn)
    .await
    .expect("could not insert worker");

    Ok(HttpResponse::Created().json(uuid))
}

#[put("/api/worker/{id}")]
async fn api_update_worker(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
    request_body: web::Json<UpdateWorker>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    worker::ActiveModel {
        id: Set(*id),
        name: Set(request_body.name.to_owned()),
        deleted: Set(false),
    }
    .save(conn)
    .await
    .expect("could not update worker");

    Ok(HttpResponse::Ok().finish())
}

#[delete("/api/delete_worker/{id}")]
async fn api_delete_worker(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    worker::ActiveModel {
        id: Set(*id),
        deleted: Set(true),
        ..Default::default()
    }
    .save(conn)
    .await
    .expect("could not delete worker");

    Ok(HttpResponse::Ok().finish())
}
