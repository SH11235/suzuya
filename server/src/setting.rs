use sea_orm::DatabaseConnection;
use serde::Deserialize;
use tera::Tera;

pub const DEFAULT_ITEMS_PER_PAGE: usize = 10;
#[derive(Debug, Clone)]
pub struct AppState {
    pub templates: Tera,
    pub conn: DatabaseConnection,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub page: Option<usize>,
    pub items_per_page: Option<usize>,
}
