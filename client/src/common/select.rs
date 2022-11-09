use serde::Serialize;
use std::fmt;

pub const PROJECT_DEFAULT: &str = "デフォルト";
pub const PROJECT_S: &str = "S案件";
pub const PROJECT_Y: &str = "Y案件";
pub const PROJECT_RE: &str = "再販";
pub fn project_type_list() -> Vec<StatusName> {
    vec![
        StatusName {
            name: String::from(PROJECT_DEFAULT),
            color: Color::WHITE,
        },
        StatusName {
            name: String::from(PROJECT_S),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from(PROJECT_Y),
            color: Color::BLUE,
        },
        StatusName {
            name: String::from(PROJECT_RE),
            color: Color::GRAY,
        },
    ]
}

pub const CATALOG_NOT_STARTED_YET: &str = "未着手";
pub const CATALOG_UNNECESSARY: &str = "不要";
pub const CATALOG_IN_PROGRESS: &str = "作成中";
pub const CATALOG_COMPLETED: &str = "作成済み";
pub fn catalog_status_list() -> Vec<StatusName> {
    vec![
        StatusName {
            name: String::from(CATALOG_NOT_STARTED_YET),
            color: Color::WHITE,
        },
        StatusName {
            name: String::from(CATALOG_UNNECESSARY),
            color: Color::GRAY,
        },
        StatusName {
            name: String::from(CATALOG_IN_PROGRESS),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from(CATALOG_COMPLETED),
            color: Color::GREEN,
        },
    ]
}

pub const ILLUST_NOT_STARTED_YET: &str = "未着手";
pub const ILLUST_WAITING_FOR_MATERIALS: &str = "素材提供待ち";
pub const ILLUST_ROUGH_IN_PROGRESS: &str = "ラフ作成中";
pub const ILLUST_ROUGH_UNDER_SUPERVISION: &str = "ラフ監修中";
pub const ILLUST_WAITING_FOR_DELIVERY: &str = "イラスト納品待ち";
pub const ILLUST_LINE_DRAWING_IN_PROGRESS: &str = "線画作成中";
pub const ILLUST_LINE_DRAWING_UNDER_SUPERVISION: &str = "線画監修中";
pub const ILLUST_COLORING_IN_PROGRESS: &str = "着彩作成中";
pub const ILLUST_COLORING_UNDER_SUPERVISION: &str = "着彩監修中";
pub const ILLUST_COMPLETED: &str = "完了";
pub fn illust_status_list() -> Vec<StatusName> {
    vec![
        StatusName {
            name: String::from(ILLUST_NOT_STARTED_YET),
            color: Color::WHITE,
        },
        StatusName {
            name: String::from(ILLUST_WAITING_FOR_MATERIALS),
            color: Color::PINK,
        },
        StatusName {
            name: String::from(ILLUST_ROUGH_IN_PROGRESS),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from(ILLUST_ROUGH_UNDER_SUPERVISION),
            color: Color::PINK,
        },
        StatusName {
            name: String::from(ILLUST_WAITING_FOR_DELIVERY),
            color: Color::PINK,
        },
        StatusName {
            name: String::from(ILLUST_LINE_DRAWING_IN_PROGRESS),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from(ILLUST_LINE_DRAWING_UNDER_SUPERVISION),
            color: Color::PINK,
        },
        StatusName {
            name: String::from(ILLUST_COLORING_IN_PROGRESS),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from(ILLUST_COLORING_UNDER_SUPERVISION),
            color: Color::PINK,
        },
        StatusName {
            name: String::from(ILLUST_COMPLETED),
            color: Color::GREEN,
        },
    ]
}

pub const DESIGN_NOT_STARTED_YET: &str = "未着手";
pub const DESIGN_WAITING_FOR_MATERIALS: &str = "素材提供待ち";
pub const DESIGN_IN_PROGRESS: &str = "デザイン作成中";
pub const DESIGN_UNDER_SUPERVISION: &str = "デザイン監修中";
pub const DESIGN_PREPARING_SUBMISSION_DATA: &str = "入稿データ作成中";
pub const DESIGN_ARRANGING_PROOFREADING: &str = "校正手配中";
pub const DESIGN_WAITING_FOR_ANOTHER_PROOFREADING: &str = "他校正待ち";
pub const DESIGN_PROOFREADING_UNDER_SUPERVISION: &str = "校正監修中";
pub const DESIGN_PROOFREADING_COMPLETED: &str = "校了";
pub const DESIGN_ORDERED: &str = "発注済み";
pub fn design_status_list() -> Vec<StatusName> {
    vec![
        StatusName {
            name: String::from(DESIGN_NOT_STARTED_YET),
            color: Color::WHITE,
        },
        StatusName {
            name: String::from(DESIGN_WAITING_FOR_MATERIALS),
            color: Color::PINK,
        },
        StatusName {
            name: String::from(DESIGN_IN_PROGRESS),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from(DESIGN_UNDER_SUPERVISION),
            color: Color::PINK,
        },
        StatusName {
            name: String::from(DESIGN_PREPARING_SUBMISSION_DATA),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from(DESIGN_ARRANGING_PROOFREADING),
            color: Color::BLUE,
        },
        StatusName {
            name: String::from(DESIGN_WAITING_FOR_ANOTHER_PROOFREADING),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from(DESIGN_PROOFREADING_UNDER_SUPERVISION),
            color: Color::PINK,
        },
        StatusName {
            name: String::from(DESIGN_PROOFREADING_COMPLETED),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from(DESIGN_ORDERED),
            color: Color::GREEN,
        },
    ]
}

pub const RESUBMISSION_NONE: &str = "";
pub const RESUBMISSION_OK: &str = "○";
pub fn resubmission_list() -> Vec<StatusName> {
    vec![
        StatusName {
            name: String::from(RESUBMISSION_NONE),
            color: Color::WHITE,
        },
        StatusName {
            name: String::from(RESUBMISSION_OK),
            color: Color::YELLOW,
        },
    ]
}

pub const LINE_UNADDRESSED: &str = "未対応";
pub const LINE_UNNECESSARY: &str = "不要";
pub const LINE_DONE: &str = "対応済";
pub fn line_list() -> Vec<StatusName> {
    vec![
        StatusName {
            name: String::from(LINE_UNADDRESSED),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from(LINE_UNNECESSARY),
            color: Color::GRAY,
        },
        StatusName {
            name: String::from(LINE_DONE),
            color: Color::GREEN,
        },
    ]
}

pub const ANNOUNCE_NOT_STARTED_YET: &str = "未着手";
pub const ANNOUNCE_UNNECESSARY: &str = "不要";
pub const ANNOUNCE_IN_PROGRESS: &str = "作成中";
pub const ANNOUNCE_UNDER_SUPERVISION: &str = "監修中";
pub const ANNOUNCE_COMPLETED: &str = "作成済み";
pub fn announce_status_list() -> Vec<StatusName> {
    vec![
        StatusName {
            name: String::from(ANNOUNCE_NOT_STARTED_YET),
            color: Color::WHITE,
        },
        StatusName {
            name: String::from(ANNOUNCE_UNNECESSARY),
            color: Color::GRAY,
        },
        StatusName {
            name: String::from(ANNOUNCE_IN_PROGRESS),
            color: Color::YELLOW,
        },
        StatusName {
            name: String::from(ANNOUNCE_UNDER_SUPERVISION),
            color: Color::PINK,
        },
        StatusName {
            name: String::from(ANNOUNCE_COMPLETED),
            color: Color::GREEN,
        },
    ]
}

pub fn get_corresponding_color(text: &str) -> Color {
    match text {
        // 重複する文言はコメントアウトしているが、色の対応は同じなので問題ない
        PROJECT_DEFAULT => Color::WHITE,
        PROJECT_S => Color::YELLOW,
        PROJECT_Y => Color::BLUE,
        PROJECT_RE => Color::GRAY,
        CATALOG_NOT_STARTED_YET => Color::WHITE,
        CATALOG_UNNECESSARY => Color::GRAY,
        CATALOG_IN_PROGRESS => Color::YELLOW,
        CATALOG_COMPLETED => Color::GREEN,
        // ILLUST_NOT_STARTED_YET => Color::WHITE,
        ILLUST_WAITING_FOR_MATERIALS => Color::PINK,
        ILLUST_ROUGH_IN_PROGRESS => Color::YELLOW,
        ILLUST_ROUGH_UNDER_SUPERVISION => Color::PINK,
        ILLUST_WAITING_FOR_DELIVERY => Color::YELLOW,
        ILLUST_LINE_DRAWING_IN_PROGRESS => Color::YELLOW,
        ILLUST_LINE_DRAWING_UNDER_SUPERVISION => Color::PINK,
        ILLUST_COLORING_IN_PROGRESS => Color::YELLOW,
        ILLUST_COLORING_UNDER_SUPERVISION => Color::PINK,
        ILLUST_COMPLETED => Color::GREEN,
        // DESIGN_NOT_STARTED_YET => Color::WHITE,
        // DESIGN_WAITING_FOR_MATERIALS => Color::PINK,
        DESIGN_IN_PROGRESS => Color::YELLOW,
        DESIGN_UNDER_SUPERVISION => Color::PINK,
        DESIGN_PREPARING_SUBMISSION_DATA => Color::YELLOW,
        DESIGN_ARRANGING_PROOFREADING => Color::BLUE,
        DESIGN_WAITING_FOR_ANOTHER_PROOFREADING => Color::YELLOW,
        DESIGN_PROOFREADING_UNDER_SUPERVISION => Color::PINK,
        DESIGN_PROOFREADING_COMPLETED => Color::YELLOW,
        DESIGN_ORDERED => Color::GREEN,
        RESUBMISSION_NONE => Color::WHITE,
        RESUBMISSION_OK => Color::YELLOW,
        LINE_UNADDRESSED => Color::YELLOW,
        // LINE_UNNECESSARY => Color::GRAY,
        LINE_DONE => Color::GREEN,
        // ANNOUNCE_NOT_STARTED_YET => Color::WHITE,
        // ANNOUNCE_UNNECESSARY => Color::GRAY,
        // ANNOUNCE_IN_PROGRESS => Color::YELLOW,
        ANNOUNCE_UNDER_SUPERVISION => Color::PINK,
        // ANNOUNCE_COMPLETED => Color::GREEN,
        _ => Color::WHITE,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Color {
    WHITE,
    PINK,
    YELLOW,
    BLUE,
    GREEN,
    GRAY,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::WHITE => write!(f, "white"),
            Color::PINK => write!(f, "pink"),
            Color::YELLOW => write!(f, "yellow"),
            Color::BLUE => write!(f, "blue"),
            Color::GREEN => write!(f, "green"),
            Color::GRAY => write!(f, "gray"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StatusName {
    pub name: String,
    pub color: Color,
}
