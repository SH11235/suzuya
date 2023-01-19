use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, web, App, HttpServer};
use sea_orm;
use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use std::env;
use suzuya::setting::AppState;
use suzuya::{item, maker, worker};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_level = env::var("RUST_LOG").unwrap_or(String::from("info"));
    std::env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    // get env vars
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    // establish connection to database and apply migrations
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    if env::var("MIGRATION").unwrap_or(String::from("false")) == String::from("true") {
        tracing::debug!("Run migration.");
        Migrator::up(&conn, None).await.unwrap();
        tracing::debug!("Finish migration.");
    }

    let state = AppState { conn };

    // create server and try to serve over socket if possible
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| -> bool {
                dotenv::dotenv().ok();
                env::var("allowed_origin")
                    .expect("allowed_origin must be set")
                    .split(",")
                    .collect::<Vec<&str>>()
                    .iter()
                    .any(|env_origin| env_origin == origin)
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .wrap(cors)
            .configure(init)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    tracing::debug!("Starting server at {}", server_url);
    server.run().await?;

    Ok(())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    // item
    cfg.service(item::api_item_list);
    cfg.service(item::api_item_new);
    cfg.service(item::api_create_items);
    cfg.service(item::api_item_edit_page);
    cfg.service(item::api_update_items);
    cfg.service(item::api_delete_item);
    cfg.service(item::api_delete_title);

    // maker
    cfg.service(maker::api_maker_list);
    cfg.service(maker::api_create_maker);
    cfg.service(maker::api_update_maker);
    cfg.service(maker::api_delete_maker);

    // worker
    cfg.service(worker::api_worker_list);
    cfg.service(worker::api_create_worker);
    cfg.service(worker::api_update_worker);
    cfg.service(worker::api_delete_worker);
}
