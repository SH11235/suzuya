use sea_orm::DatabaseConnection;
use tera::Tera;

#[derive(Debug, Clone)]
pub struct AppState {
    pub templates: Tera,
    pub conn: DatabaseConnection,
}
