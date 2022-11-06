use std::collections::HashMap;

use super::api::RESUBMISSION_OK;

pub fn project_type_list() -> Vec<String> {
    vec![
        String::from("デフォルト"),
        String::from("A案件"),
        String::from("Y案件"),
        String::from("再販"),
    ]
}

pub fn catalog_status_list() -> Vec<String> {
    vec![
        String::from("未着手"),
        String::from("不要"),
        String::from("作成中"),
        String::from("作成済み"),
    ]
}

pub fn illust_status_list() -> Vec<String> {
    vec![
        String::from("未着手"),
        String::from("素材提供待ち"),
        String::from("ラフ作成中"),
        String::from("ラフ監修中"),
        String::from("イラスト納品待ち"),
        String::from("線画作成中"),
        String::from("線画監修中"),
        String::from("着彩作成中"),
        String::from("着彩監修中"),
        String::from("完了"),
    ]
}

pub fn design_status_list() -> Vec<String> {
    vec![
        String::from("未着手"),
        String::from("素材提供待ち"),
        String::from("デザイン作成中"),
        String::from("デザイン監修中"),
        String::from("入稿データ作成中"),
        String::from("校正手配中"),
        String::from("他校正待ち"),
        String::from("校正監修中"),
        String::from("校了"),
        String::from("発注済み"),
    ]
}

pub fn resubmission_list() -> Vec<String> {
    vec![
        String::from(""),
        String::from(RESUBMISSION_OK),
    ]
}

pub fn line_list() -> Vec<String> {
    vec![
        String::from("未対応"),
        String::from("不要"),
        String::from("対応済"),
    ]
}

pub fn announce_status_list() -> Vec<String> {
    vec![
        String::from("未着手"),
        String::from("不要"),
        String::from("作成中"),
        String::from("監修中"),
        String::from("作成済み"),
    ]
}

pub fn color_list() -> Vec<String> {
    vec![
        String::from("white"),
        String::from("pink"),
        String::from("yellow"),
        String::from("light-blue"),
        String::from("green"),
        String::from("gray"),
    ]
}

pub fn project_type_color_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert(String::from("デフォルト"), String::from("white"));
    map.insert(String::from("A案件"), String::from("yellow"));
    map.insert(String::from("Y案件"), String::from("light-blue"));
    map.insert(String::from("再販"), String::from("gray"));
    map
}
