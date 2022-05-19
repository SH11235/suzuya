use actix_files::Files as Fs;
use actix_web::{middleware, web, App, HttpServer};
use entity::sea_orm;
use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use std::env;
use suzuya::setting::AppState;
use suzuya::{item, maker, user};
use tera::Tera;

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
    // item
    cfg.service(item::item_list);
    cfg.service(item::new_item);
    cfg.service(item::create_items);
    cfg.service(item::edit_items);
    cfg.service(item::update_items);

    // maker
    cfg.service(maker::maker_list);
    cfg.service(maker::new_maker);
    cfg.service(maker::create_maker);
    cfg.service(maker::edit_maker);
    cfg.service(maker::update_maker);
    cfg.service(maker::delete_maker);

    // user
    cfg.service(user::user_list);
    cfg.service(user::new_user);
    cfg.service(user::create_user);
    cfg.service(user::edit_user);
    cfg.service(user::update_user);
    cfg.service(user::delete_user);
}
