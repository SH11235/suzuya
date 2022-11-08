// use std::collections::HashMap;

use super::api::RESUBMISSION_OK;

pub fn project_type_list() -> Vec<String> {
    vec![
        String::from("デフォルト"),
        String::from("S案件"),
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

// pub fn color_list() -> Vec<String> {
//     vec![
//         String::from("white"),
//         String::from("pink"),
//         String::from("yellow"),
//         String::from("light-blue"),
//         String::from("green"),
//         String::from("gray"),
//     ]
// }

// pub fn project_type_color_map() -> HashMap<String, String> {
//     let mut map = HashMap::new();
//     map.insert(String::from("デフォルト"), String::from("white"));
//     map.insert(String::from("S案件"), String::from("yellow"));
//     map.insert(String::from("Y案件"), String::from("light-blue"));
//     map.insert(String::from("再販"), String::from("gray"));
//     map
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct StatusName {
//     pub name: String,
//     pub color: Color,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum Color {
//     WHITE,
//     PINK,
//     YELLOW,
//     LIGHTBLUE,
//     GREEN,
//     GRAY,
// }

// pub fn project_type_list() -> Vec<StatusName> {
//     vec![
//         StatusName {
//             name: String::from("デフォルト"),
//             color: Color::WHITE,
//         },
//         StatusName {
//             name: String::from("S案件"),
//             color: Color::YELLOW,
//         },
//         StatusName {
//             name: String::from("Y案件"),
//             color: Color::LIGHTBLUE,
//         },
//         StatusName {
//             name: String::from("再販"),
//             color: Color::GRAY,
//         },
//     ]
// }

// pub fn illust_status_list() -> Vec<StatusName> {
//     vec![
//         StatusName {
//             name: String::from("未着手"),
//             color: Color::WHITE,
//         },
//         StatusName {
//             name: String::from("素材提供待ち"),
//             color: Color::PINK,
//         },
//         StatusName {
//             name: String::from("ラフ作成中"),
//             color: Color::YELLOW,
//         },
//         StatusName {
//             name: String::from("ラフ監修中"),
//             color: Color::PINK,
//         },
//         StatusName {
//             name: String::from("イラスト納品待ち"),
//             color: Color::PINK,
//         },
//         StatusName {
//             name: String::from("線画作成中"),
//             color: Color::YELLOW,
//         },
//         StatusName {
//             name: String::from("線画監修中"),
//             color: Color::PINK,
//         },
//         StatusName {
//             name: String::from("着彩作成中"),
//             color: Color::YELLOW,
//         },
//         StatusName {
//             name: String::from("着彩監修中"),
//             color: Color::PINK,
//         },
//         StatusName {
//             name: String::from("完了"),
//             color: Color::GREEN,
//         },
//     ]
// }

// pub fn design_status_list() -> Vec<StatusName> {
//     vec![
//         StatusName {
//             name: String::from("未着手"),
//             color: Color::WHITE,
//         },
//         StatusName {
//             name: String::from("素材提供待ち"),
//             color: Color::PINK,
//         },
//         StatusName {
//             name: String::from("デザイン作成中"),
//             color: Color::YELLOW,
//         },
//         StatusName {
//             name: String::from("デザイン監修中"),
//             color: Color::PINK,
//         },
//         StatusName {
//             name: String::from("入稿データ作成中"),
//             color: Color::YELLOW,
//         },
//         StatusName {
//             name: String::from("校正手配中"),
//             color: Color::LIGHTBLUE,
//         },
//         StatusName {
//             name: String::from("他校正待ち"),
//             color: Color::YELLOW,
//         },
//         StatusName {
//             name: String::from("校正監修中"),
//             color: Color::PINK,
//         },
//         StatusName {
//             name: String::from("校了"),
//             color: Color::YELLOW,
//         },
//         StatusName {
//             name: String::from("発注済み"),
//             color: Color::GREEN,
//         },
//     ]
// }

// pub fn catalog_status_list() -> Vec<StatusName> {
//     vec![
//         StatusName {
//             name: String::from("未着手"),
//             color: Color::WHITE,
//         },
//         StatusName {
//             name: String::from("不要"),
//             color: Color::GRAY,
//         },
//         StatusName {
//             name: String::from("作成中"),
//             color: Color::YELLOW,
//         },
//         StatusName {
//             name: String::from("作成済み"),
//             color: Color::GREEN,
//         },
//     ]
// }

// pub fn announce_status_list() -> Vec<StatusName> {
//     vec![
//         StatusName {
//             name: String::from("未着手"),
//             color: Color::WHITE,
//         },
//         StatusName {
//             name: String::from("不要"),
//             color: Color::GRAY,
//         },
//         StatusName {
//             name: String::from("作成中"),
//             color: Color::YELLOW,
//         },
//         StatusName {
//             name: String::from("監修中"),
//             color: Color::PINK,
//         },
//         StatusName {
//             name: String::from("作成済み"),
//             color: Color::GREEN,
//         },
//     ]
// }

