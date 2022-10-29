use crate::setting::AppState;
use actix_web::{delete, error, get, post, put, web, Error, HttpResponse, Result};
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

#[get("/maker")]
async fn maker_list(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let conn = &data.conn;
    let datas = Maker::find()
        .filter(maker::Column::Deleted.eq(false))
        .order_by_asc(maker::Column::Id)
        .all(conn)
        .await
        .expect("could not retrieve datas");
    let mut ctx = tera::Context::new();
    let h1 = "メーカーコード";
    let path = "maker";
    ctx.insert("datas", &datas);
    ctx.insert("h1", &h1);
    ctx.insert("path", &path);

    let body = template
        .render("maker/maker_list.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
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

#[get("/new_maker")]
async fn new_maker(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let mut ctx = tera::Context::new();
    let path = "maker";
    ctx.insert("path", &path);
    let body = template
        .render("maker/new_maker.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/new_maker")]
async fn create_maker(
    data: web::Data<AppState>,
    post_form: web::Form<InputNewMaker>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = post_form.into_inner();

    maker::ActiveModel {
        code_name: Set(form.code_name.to_owned()),
        ..Default::default()
    }
    .insert(conn)
    .await
    .expect("could not insert maker");

    Ok(HttpResponse::Found()
        .append_header(("location", "/new_maker"))
        .finish())
}

#[post("/api/new_maker")]
async fn api_create_maker(
    data: web::Data<AppState>,
    request_body: web::Json<InputNewMaker>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let uuid = Uuid::new_v4();

    maker::ActiveModel {
        id: Set(uuid.clone()),
        code_name: Set(request_body.code_name.to_owned()),
        ..Default::default()
    }
    .insert(conn)
    .await
    .expect("could not insert maker");

    Ok(HttpResponse::Created().json(uuid))
}

#[get("/maker/{id}")]
async fn edit_maker(data: web::Data<AppState>, id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let template = &data.templates;

    let maker: maker::Model = Maker::find_by_id(id.into_inner())
        .one(conn)
        .await
        .expect("could not find maker")
        .unwrap();

    let mut ctx = tera::Context::new();
    let path = "maker";
    ctx.insert("maker", &maker);
    ctx.insert("path", &path);

    let body = template
        .render("maker/edit_maker.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/maker/{id}")]
async fn update_maker(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
    post_form: web::Form<UpdateMaker>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = post_form.into_inner();

    maker::ActiveModel {
        id: Set(id.into_inner()),
        code_name: Set(form.code_name.to_owned()),
        ..Default::default()
    }
    .save(conn)
    .await
    .expect("could not edit maker");

    Ok(HttpResponse::Found()
        .append_header(("location", "/maker"))
        .finish())
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

#[post("/delete_maker/{id}")]
async fn delete_maker(
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

    Ok(HttpResponse::Found()
        .append_header(("location", "/maker"))
        .finish())
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
