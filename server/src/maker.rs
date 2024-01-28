use crate::setting::AppState;
use actix_web::{delete, get, post, put, web, Error, HttpResponse, Result};
use entity::maker;
use entity::maker::Entity as Maker;
use sea_orm::{entity::*, prelude::Uuid, query::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct InputNewMaker {
    code_name: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct UpdateMaker {
    code_name: String,
}

#[get("/api/maker_list")]
async fn api_maker_list(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let datas = Maker::find()
        .filter(maker::Column::Deleted.eq(false))
        .order_by_asc(maker::Column::CodeName)
        .all(conn)
        .await
        .expect("could not retrieve datas");

    Ok(HttpResponse::Ok().json(datas))
}

#[post("/api/new_maker")]
async fn api_create_maker(
    data: web::Data<AppState>,
    request_body: web::Json<InputNewMaker>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let uuid = Uuid::new_v4();

    maker::ActiveModel {
        id: Set(uuid),
        code_name: Set(request_body.code_name.to_owned()),
        ..Default::default()
    }
    .insert(conn)
    .await
    .expect("could not insert maker");

    Ok(HttpResponse::Created().json(uuid))
}

#[put("/api/maker/{id}")]
async fn api_update_maker(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
    request_body: web::Json<UpdateMaker>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    maker::ActiveModel {
        id: Set(id.into_inner()),
        code_name: Set(request_body.code_name.to_owned()),
        ..Default::default()
    }
    .save(conn)
    .await
    .expect("could not update maker");

    Ok(HttpResponse::Ok().finish())
}

#[delete("/api/delete_maker/{id}")]
async fn api_delete_maker(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    maker::ActiveModel {
        id: Set(*id),
        deleted: Set(true),
        ..Default::default()
    }
    .save(conn)
    .await
    .expect("could not delete maker");

    Ok(HttpResponse::Ok().finish())
}
