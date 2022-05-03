use crate::setting::AppState;
use actix_web::{error, get, post, web, Error, HttpResponse, Result};
use chrono::Local;
use entity::user;
use entity::user::Entity as User;
use sea_orm::{entity::*, query::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct InputNewUser {
    name: String,
    description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct UpdateUser {
    name: String,
    description: Option<String>,
}

#[get("/user")]
async fn user_list(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let conn = &data.conn;
    let datas = User::find()
        .order_by_asc(user::Column::Id)
        .filter(user::Column::Deleted.eq(false))
        .all(conn)
        .await
        .expect("could not retrieve datas");
    let mut ctx = tera::Context::new();
    let h1 = "関係者";
    let path = "user";

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
        .map(|user| ViewData {
            id: user.id,
            name: user.name.clone(),
            description: user.description.clone(),
            created_at: user
                .created_at
                .format("%Y年%m月%d日 <br> %H時%M分%S秒")
                .to_string(),
            updated_at: user
                .updated_at
                .format("%Y年%m月%d日 <br> %H時%M分%S秒")
                .to_string(),
        })
        .collect::<Vec<ViewData>>();

    ctx.insert("datas", &view_datas);
    ctx.insert("h1", &h1);
    ctx.insert("path", &path);

    let body = template
        .render("user_list.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/new_user")]
async fn new_user(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let ctx = tera::Context::new();
    let body = template
        .render("new_user.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/new_user")]
async fn create_user(
    data: web::Data<AppState>,
    post_form: web::Form<InputNewUser>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = post_form.into_inner();

    let date = Local::now();

    user::ActiveModel {
        name: Set(form.name.to_owned()),
        description: Set(form.description.to_owned()),
        created_at: Set(date),
        updated_at: Set(date),
        ..Default::default()
    }
    .insert(conn)
    .await
    .expect("could not insert user");

    Ok(HttpResponse::Found()
        .append_header(("location", "/new_user"))
        .finish())
}

#[get("/user/{id}")]
async fn edit_user(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let template = &data.templates;

    let user: user::Model = User::find_by_id(id.into_inner())
        .one(conn)
        .await
        .expect("could not find user")
        .unwrap();

    let mut ctx = tera::Context::new();
    let path = "user";
    ctx.insert("user", &user);
    ctx.insert("path", &path);

    let body = template
        .render("edit_user.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/user/{id}")]
async fn update_user(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    post_form: web::Form<UpdateUser>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = post_form.into_inner();

    let date = Local::now();

    user::ActiveModel {
        id: Set(*id),
        name: Set(form.name.to_owned()),
        description: Set(form.description.to_owned()),
        updated_at: Set(date),
        ..Default::default()
    }
    .save(conn)
    .await
    .expect("could not edit user");

    Ok(HttpResponse::Found()
        .append_header(("location", "/user"))
        .finish())
}

#[post("/delete_user/{id}")]
async fn delete_user(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    user::ActiveModel {
        id: Set(*id),
        deleted: Set(true),
        ..Default::default()
    }
    .save(conn)
    .await
    .expect("could not delete user");

    Ok(HttpResponse::Found()
        .append_header(("location", "/user"))
        .finish())
}
