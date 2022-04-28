use crate::setting::{AppState, Params, DEFAULT_ITEMS_PER_PAGE};
use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use entity::maker;
use entity::maker::Entity as Maker;
use sea_orm::{entity::*, query::*};

#[get("/maker")]
async fn maker_list(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let conn = &data.conn;

    // get params
    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();

    let page = params.page.unwrap_or(1);
    let items_per_page = params.items_per_page.unwrap_or(DEFAULT_ITEMS_PER_PAGE);
    let paginator = Maker::find()
        .order_by_asc(maker::Column::Id)
        .paginate(conn, items_per_page);
    let num_pages = paginator.num_pages().await.ok().unwrap();

    let datas = paginator
        .fetch_page(page - 1)
        .await
        .expect("could not retrieve datas");
    let mut ctx = tera::Context::new();
    let h1 = "メーカーコード";
    let path = "maker";
    ctx.insert("datas", &datas);
    ctx.insert("page", &page);
    ctx.insert("h1", &h1);
    ctx.insert("path", &path);
    ctx.insert("items_per_page", &items_per_page);
    ctx.insert("num_pages", &num_pages);

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
    post_form: web::Form<maker::Model>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let count = Maker::find()
        .column(maker::Column::Id)
        .all(conn)
        .await
        .unwrap()
        .len() as i32;
    let mut form = post_form.into_inner();
    form.id = count + 1;

    maker::ActiveModel {
        id: Set(form.id),
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
    post_form: web::Form<maker::Model>,
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
