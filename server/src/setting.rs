use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Debug, Clone)]
pub struct AppState {
    pub templates: Tera,
    pub conn: DatabaseConnection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusName {
    pub name: String,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Color {
    WHITE,
    PINK,
    YELLOW,
    LIGHTBLUE,
    GREEN,
    GRAY,
}

pub fn project_type_list() -> Vec<StatusName> {
    vec![
        StatusName {
            name: String::from("デフォルト"),
            color: Color::WHITE,
        },
        StatusName {
            name: String::from("A案件"),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from("Y案件"),
            color: Color::LIGHTBLUE,
        },
        StatusName {
            name: String::from("再販"),
            color: Color::GRAY,
        },
    ]
}

pub fn illust_status_list() -> Vec<StatusName> {
    vec![
        StatusName {
            name: String::from("未着手"),
            color: Color::WHITE,
        },
        StatusName {
            name: String::from("素材提供待ち"),
            color: Color::PINK,
        },
        StatusName {
            name: String::from("ラフ作成中"),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from("ラフ監修中"),
            color: Color::PINK,
        },
        StatusName {
            name: String::from("イラスト納品待ち"),
            color: Color::PINK,
        },
        StatusName {
            name: String::from("線画作成中"),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from("線画監修中"),
            color: Color::PINK,
        },
        StatusName {
            name: String::from("着彩作成中"),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from("着彩監修中"),
            color: Color::PINK,
        },
        StatusName {
            name: String::from("完了"),
            color: Color::GREEN,
        },
    ]
}

pub fn design_status_list() -> Vec<StatusName> {
    vec![
        StatusName {
            name: String::from("未着手"),
            color: Color::WHITE,
        },
        StatusName {
            name: String::from("素材提供待ち"),
            color: Color::PINK,
        },
        StatusName {
            name: String::from("デザイン作成中"),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from("デザイン監修中"),
            color: Color::PINK,
        },
        StatusName {
            name: String::from("入稿データ作成中"),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from("校正手配中"),
            color: Color::LIGHTBLUE,
        },
        StatusName {
            name: String::from("他校正待ち"),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from("校正監修中"),
            color: Color::PINK,
        },
        StatusName {
            name: String::from("校了"),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from("発注済み"),
            color: Color::GREEN,
        },
    ]
}

pub fn catalog_status_list() -> Vec<StatusName> {
    vec![
        StatusName {
            name: String::from("未着手"),
            color: Color::WHITE,
        },
        StatusName {
            name: String::from("不要"),
            color: Color::GRAY,
        },
        StatusName {
            name: String::from("作成中"),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from("作成済み"),
            color: Color::GREEN,
        },
    ]
}

pub fn announce_status_list() -> Vec<StatusName> {
    vec![
        StatusName {
            name: String::from("未着手"),
            color: Color::WHITE,
        },
        StatusName {
            name: String::from("不要"),
            color: Color::GRAY,
        },
        StatusName {
            name: String::from("作成中"),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from("監修中"),
            color: Color::PINK,
        },
        StatusName {
            name: String::from("作成済み"),
            color: Color::GREEN,
        },
    ]
}
