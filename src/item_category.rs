use crate::setting::{AppState, Params, DEFAULT_ITEMS_PER_PAGE};
use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use entity::item_category;
use entity::item_category::Entity as ItemCategory;
use sea_orm::{entity::*, query::*};

#[get("/item_category")]
async fn item_category_list(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let conn = &data.conn;

    // get params
    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();

    let page = params.page.unwrap_or(1);
    let items_per_page = params.items_per_page.unwrap_or(DEFAULT_ITEMS_PER_PAGE);
    let paginator = ItemCategory::find()
        .order_by_asc(item_category::Column::Id)
        .paginate(conn, items_per_page);
    let num_pages = paginator.num_pages().await.ok().unwrap();

    let datas = paginator
        .fetch_page(page - 1)
        .await
        .expect("could not retrieve datas");
    let mut ctx = tera::Context::new();
    let h1 = "アイテムカテゴリー";
    let path = "item_category";
    ctx.insert("datas", &datas);
    ctx.insert("page", &page);
    ctx.insert("h1", &h1);
    ctx.insert("path", &path);
    ctx.insert("items_per_page", &items_per_page);
    ctx.insert("num_pages", &num_pages);

    let body = template
        .render("item_category_list.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/new_item_category")]
async fn new_item_category(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let mut ctx = tera::Context::new();
    let h4 = "アイテムカテゴリー登録";
    let path = "item_category";
    ctx.insert("h4", &h4);
    ctx.insert("path", &path);
    let body = template
        .render("new_item_category.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/new_item_category")]
async fn create_item_category(
    data: web::Data<AppState>,
    post_form: web::Form<item_category::Model>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let count = ItemCategory::find()
        .column(item_category::Column::Id)
        .all(conn)
        .await
        .unwrap()
        .len() as i32;
    let mut form = post_form.into_inner();
    form.id = count + 1;

    item_category::ActiveModel {
        id: Set(form.id),
        name: Set(form.name.to_owned()),
        code_name: Set(form.code_name.to_owned()),
        ..Default::default()
    }
    .insert(conn)
    .await
    .expect("could not insert item category");

    Ok(HttpResponse::Found()
        .append_header(("location", "/new_item_category"))
        .finish())
}

#[get("/item_category/{id}")]
async fn edit_item_category(
    data: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let template = &data.templates;

    let item_category: item_category::Model = ItemCategory::find_by_id(id.into_inner())
        .one(conn)
        .await
        .expect("could not find item category")
        .unwrap();

    let mut ctx = tera::Context::new();
    let path = "item_category";
    ctx.insert("item_category", &item_category);
    ctx.insert("path", &path);

    let body = template
        .render("edit_item_category.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/item_category/{id}")]
async fn update_item_category(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    post_form: web::Form<item_category::Model>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = post_form.into_inner();

    item_category::ActiveModel {
        id: Set(id.into_inner()),
        name: Set(form.name.to_owned()),
        code_name: Set(form.code_name.to_owned()),
    }
    .save(conn)
    .await
    .expect("could not edit item category");

    Ok(HttpResponse::Found()
        .append_header(("location", "/item_category"))
        .finish())
}

#[post("/delete_item_category/{id}")]
async fn delete_item_category(
    data: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let post: item_category::ActiveModel = ItemCategory::find_by_id(id.into_inner())
        .one(conn)
        .await
        .unwrap()
        .unwrap()
        .into();

    post.delete(conn).await.unwrap();

    Ok(HttpResponse::Found()
        .append_header(("location", "/item_category"))
        .finish())
}
