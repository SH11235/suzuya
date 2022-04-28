use crate::setting::{AppState, Params, DEFAULT_ITEMS_PER_PAGE};
use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use chrono::Local;
use entity::user;
use entity::user::Entity as User;
use sea_orm::{entity::*, query::*};
use serde::{Deserialize, Serialize};

#[get("/user")]
async fn user_list(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let conn = &data.conn;

    // get params
    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();

    let page = params.page.unwrap_or(1);
    let items_per_page = params.items_per_page.unwrap_or(DEFAULT_ITEMS_PER_PAGE);
    let paginator = User::find()
        .order_by_asc(user::Column::Id)
        .filter(user::Column::Deleted.eq(false))
        .paginate(conn, items_per_page);
    let num_pages = paginator.num_pages().await.ok().unwrap();

    let datas = paginator
        .fetch_page(page - 1)
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
    ctx.insert("page", &page);
    ctx.insert("h1", &h1);
    ctx.insert("path", &path);
    ctx.insert("items_per_page", &items_per_page);
    ctx.insert("num_pages", &num_pages);

    let body = template
        .render("user_list.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/new_user")]
async fn new_user(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let mut ctx = tera::Context::new();
    let h4 = "関係者登録";
    let path = "person";
    ctx.insert("h4", &h4);
    ctx.insert("path", &path);
    let body = template
        .render("new_user.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/new_user")]
async fn create_user(
    data: web::Data<AppState>,
    post_form: web::Form<user::Model>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let count = User::find()
        .column(user::Column::Id)
        .order_by_desc(user::Column::Id)
        .one(conn)
        .await
        .unwrap();
    let count = match count {
        Some(user) => user.id,
        None => 0,
    };
    let mut form = post_form.into_inner();
    form.id = count + 1;

    let date = Local::now();
    form.created_at = date;
    form.updated_at = date;

    user::ActiveModel {
        id: Set(form.id),
        name: Set(form.name.to_owned()),
        description: Set(form.description.to_owned()),
        created_at: Set(form.created_at.to_owned()),
        updated_at: Set(form.updated_at.to_owned()),
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
    post_form: web::Form<user::Model>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let mut form = post_form.into_inner();

    let created_at = User::find_by_id(id.clone())
        .column(user::Column::CreatedAt)
        .one(conn)
        .await
        .unwrap()
        .unwrap()
        .created_at;
    form.created_at = created_at;

    let date = Local::now();
    form.updated_at = date;

    user::ActiveModel {
        id: Set(*id),
        name: Set(form.name.to_owned()),
        description: Set(form.description.to_owned()),
        deleted: Set(false),
        created_at: Set(form.created_at.to_owned()),
        updated_at: Set(form.updated_at.to_owned()),
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

    let user = User::find_by_id(id.clone())
        .one(conn)
        .await
        .unwrap()
        .unwrap();

    user::ActiveModel {
        id: Set(*id),
        name: Set(user.name.to_owned()),
        description: Set(user.description.to_owned()),
        deleted: Set(true),
        created_at: Set(user.created_at.to_owned()),
        updated_at: Set(user.updated_at.to_owned()),
    }
    .save(conn)
    .await
    .expect("could not delete user");

    Ok(HttpResponse::Found()
        .append_header(("location", "/user"))
        .finish())
}
