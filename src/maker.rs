use crate::setting::AppState;
use actix_web::{error, get, post, web, Error, HttpResponse, Result};
use entity::maker;
use entity::maker::Entity as Maker;
use sea_orm::{entity::*, query::*};
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
        .render("maker_list.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/new_maker")]
async fn new_maker(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let mut ctx = tera::Context::new();
    let h4 = "メーカーコード登録";
    let path = "maker";
    ctx.insert("h4", &h4);
    ctx.insert("path", &path);
    let body = template
        .render("new_maker.html.tera", &ctx)
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

#[get("/maker/{id}")]
async fn edit_maker(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
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
        .render("edit_maker.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/maker/{id}")]
async fn update_maker(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    post_form: web::Form<UpdateMaker>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = post_form.into_inner();

    maker::ActiveModel {
        id: Set(id.into_inner()),
        code_name: Set(form.code_name.to_owned()),
    }
    .save(conn)
    .await
    .expect("could not edit maker");

    Ok(HttpResponse::Found()
        .append_header(("location", "/maker"))
        .finish())
}

#[post("/delete_maker/{id}")]
async fn delete_maker(
    data: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let post: maker::ActiveModel = Maker::find_by_id(id.into_inner())
        .one(conn)
        .await
        .unwrap()
        .unwrap()
        .into();

    post.delete(conn).await.unwrap();

    Ok(HttpResponse::Found()
        .append_header(("location", "/maker"))
        .finish())
}
