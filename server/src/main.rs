use actix_files::Files as Fs;
use actix_web::{middleware, web, App, HttpServer};
use entity::sea_orm;
use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use std::env;
use tera::Tera;

const DEFAULT_ITEMS_PER_PAGE: usize = 10;

#[derive(Debug, Clone)]
struct AppState {
    templates: Tera,
    conn: DatabaseConnection,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    page: Option<usize>,
    items_per_page: Option<usize>,
}

mod item_category_mod {
    use crate::sea_orm::{entity::*, query::*};
    use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
    use entity::item_category;
    use entity::item_category::Entity as ItemCategory;
    #[get("/item_category")]
    async fn item_category_list(
        req: HttpRequest,
        data: web::Data<super::AppState>,
    ) -> Result<HttpResponse, Error> {
        let template = &data.templates;
        let conn = &data.conn;

        // get params
        let params = web::Query::<super::Params>::from_query(req.query_string()).unwrap();

        let page = params.page.unwrap_or(1);
        let items_per_page = params
            .items_per_page
            .unwrap_or(super::DEFAULT_ITEMS_PER_PAGE);
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
            .render("data.html.tera", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;
        Ok(HttpResponse::Ok().content_type("text/html").body(body))
    }

    #[get("/new_item_category")]
    async fn new_item_category(data: web::Data<super::AppState>) -> Result<HttpResponse, Error> {
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
        data: web::Data<super::AppState>,
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
        data: web::Data<super::AppState>,
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
        data: web::Data<super::AppState>,
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
        data: web::Data<super::AppState>,
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
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    // get env vars
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    // establish connection to database and apply migrations
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    match env::var("MIGRATION") {
        Ok(flag) => {
            if flag == String::from("true") {
                println!("Run migration.");
                Migrator::up(&conn, None).await.unwrap();
                println!("Finish migration.");
            }
        }
        Err(_) => (),
    }
    // load tera templates and build app state
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    let state = AppState { templates, conn };

    // create server and try to serve over socket if possible
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .service(Fs::new("/static", "./static"))
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .configure(init)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {}", server_url);
    server.run().await?;

    Ok(())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(item_category_mod::item_category_list);
    cfg.service(item_category_mod::new_item_category);
    cfg.service(item_category_mod::create_item_category);
    cfg.service(item_category_mod::edit_item_category);
    cfg.service(item_category_mod::update_item_category);
    cfg.service(item_category_mod::delete_item_category);
}
