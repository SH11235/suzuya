use crate::setting::AppState;
use actix_identity::Identity;
use actix_web::{error, get, post, web, Error, HttpResponse, Result};

#[get("/login")]
async fn index(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let ctx = tera::Context::new();
    let body = template
        .render("login.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/login")]
async fn login(id: Identity, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    // remember identity
    id.remember("User1".to_owned());
    let template = &data.templates;
    let ctx = tera::Context::new();
    let body = template
        .render("index.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/logout")]
async fn logout(id: Identity) -> HttpResponse {
    // remove identity
    id.forget();
    HttpResponse::Ok().finish()
}
