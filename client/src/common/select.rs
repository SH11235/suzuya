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
            color: Color::White,
        },
        StatusName {
            name: String::from(PROJECT_S),
            color: Color::Yellow,
        },
        StatusName {
            name: String::from(PROJECT_Y),
            color: Color::Blue,
        },
        StatusName {
            name: String::from(PROJECT_RE),
            color: Color::Gray,
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
            color: Color::White,
        },
        StatusName {
            name: String::from(CATALOG_UNNECESSARY),
            color: Color::Gray,
        },
        StatusName {
            name: String::from(CATALOG_IN_PROGRESS),
            color: Color::Yellow,
        },
        StatusName {
            name: String::from(CATALOG_COMPLETED),
            color: Color::Green,
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
            color: Color::White,
        },
        StatusName {
            name: String::from(ILLUST_WAITING_FOR_MATERIALS),
            color: Color::Pink,
        },
        StatusName {
            name: String::from(ILLUST_ROUGH_IN_PROGRESS),
            color: Color::Yellow,
        },
        StatusName {
            name: String::from(ILLUST_ROUGH_UNDER_SUPERVISION),
            color: Color::Pink,
        },
        StatusName {
            name: String::from(ILLUST_WAITING_FOR_DELIVERY),
            color: Color::Pink,
        },
        StatusName {
            name: String::from(ILLUST_LINE_DRAWING_IN_PROGRESS),
            color: Color::Yellow,
        },
        StatusName {
            name: String::from(ILLUST_LINE_DRAWING_UNDER_SUPERVISION),
            color: Color::Pink,
        },
        StatusName {
            name: String::from(ILLUST_COLORING_IN_PROGRESS),
            color: Color::Yellow,
        },
        StatusName {
            name: String::from(ILLUST_COLORING_UNDER_SUPERVISION),
            color: Color::Pink,
        },
        StatusName {
            name: String::from(ILLUST_COMPLETED),
            color: Color::Green,
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
            color: Color::White,
        },
        StatusName {
            name: String::from(DESIGN_WAITING_FOR_MATERIALS),
            color: Color::Pink,
        },
        StatusName {
            name: String::from(DESIGN_IN_PROGRESS),
            color: Color::Yellow,
        },
        StatusName {
            name: String::from(DESIGN_UNDER_SUPERVISION),
            color: Color::Pink,
        },
        StatusName {
            name: String::from(DESIGN_PREPARING_SUBMISSION_DATA),
            color: Color::Yellow,
        },
        StatusName {
            name: String::from(DESIGN_ARRANGING_PROOFREADING),
            color: Color::Blue,
        },
        StatusName {
            name: String::from(DESIGN_WAITING_FOR_ANOTHER_PROOFREADING),
            color: Color::Yellow,
        },
        StatusName {
            name: String::from(DESIGN_PROOFREADING_UNDER_SUPERVISION),
            color: Color::Pink,
        },
        StatusName {
            name: String::from(DESIGN_PROOFREADING_COMPLETED),
            color: Color::Yellow,
        },
        StatusName {
            name: String::from(DESIGN_ORDERED),
            color: Color::Green,
        },
    ]
}

pub const RESUBMISSION_NONE: &str = "";
pub const RESUBMISSION_OK: &str = "○";
pub fn resubmission_list() -> Vec<StatusName> {
    vec![
        StatusName {
            name: String::from(RESUBMISSION_NONE),
            color: Color::White,
        },
        StatusName {
            name: String::from(RESUBMISSION_OK),
            color: Color::Yellow,
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
            color: Color::Yellow,
        },
        StatusName {
            name: String::from(LINE_UNNECESSARY),
            color: Color::Gray,
        },
        StatusName {
            name: String::from(LINE_DONE),
            color: Color::Green,
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
            color: Color::White,
        },
        StatusName {
            name: String::from(ANNOUNCE_UNNECESSARY),
            color: Color::Gray,
        },
        StatusName {
            name: String::from(ANNOUNCE_IN_PROGRESS),
            color: Color::Yellow,
        },
        StatusName {
            name: String::from(ANNOUNCE_UNDER_SUPERVISION),
            color: Color::Pink,
        },
        StatusName {
            name: String::from(ANNOUNCE_COMPLETED),
            color: Color::Green,
        },
    ]
}

pub fn get_corresponding_color(text: &str) -> Color {
    match text {
        // 重複する文言はコメントアウトしているが、色の対応は同じなので問題ない
        PROJECT_DEFAULT => Color::White,
        PROJECT_S => Color::Yellow,
        PROJECT_Y => Color::Blue,
        PROJECT_RE => Color::Gray,
        CATALOG_NOT_STARTED_YET => Color::White,
        CATALOG_UNNECESSARY => Color::Gray,
        CATALOG_IN_PROGRESS => Color::Yellow,
        CATALOG_COMPLETED => Color::Green,
        // ILLUST_NOT_STARTED_YET => Color::WHITE,
        ILLUST_WAITING_FOR_MATERIALS => Color::Pink,
        ILLUST_ROUGH_IN_PROGRESS => Color::Yellow,
        ILLUST_ROUGH_UNDER_SUPERVISION => Color::Pink,
        ILLUST_WAITING_FOR_DELIVERY => Color::Yellow,
        ILLUST_LINE_DRAWING_IN_PROGRESS => Color::Yellow,
        ILLUST_LINE_DRAWING_UNDER_SUPERVISION => Color::Pink,
        ILLUST_COLORING_IN_PROGRESS => Color::Yellow,
        ILLUST_COLORING_UNDER_SUPERVISION => Color::Pink,
        ILLUST_COMPLETED => Color::Green,
        // DESIGN_NOT_STARTED_YET => Color::WHITE,
        // DESIGN_WAITING_FOR_MATERIALS => Color::PINK,
        DESIGN_IN_PROGRESS => Color::Yellow,
        DESIGN_UNDER_SUPERVISION => Color::Pink,
        DESIGN_PREPARING_SUBMISSION_DATA => Color::Yellow,
        DESIGN_ARRANGING_PROOFREADING => Color::Blue,
        DESIGN_WAITING_FOR_ANOTHER_PROOFREADING => Color::Yellow,
        DESIGN_PROOFREADING_UNDER_SUPERVISION => Color::Pink,
        DESIGN_PROOFREADING_COMPLETED => Color::Yellow,
        DESIGN_ORDERED => Color::Green,
        RESUBMISSION_NONE => Color::White,
        RESUBMISSION_OK => Color::Yellow,
        LINE_UNADDRESSED => Color::Yellow,
        // LINE_UNNECESSARY => Color::GRAY,
        LINE_DONE => Color::Green,
        // ANNOUNCE_NOT_STARTED_YET => Color::WHITE,
        // ANNOUNCE_UNNECESSARY => Color::GRAY,
        // ANNOUNCE_IN_PROGRESS => Color::YELLOW,
        ANNOUNCE_UNDER_SUPERVISION => Color::Pink,
        // ANNOUNCE_COMPLETED => Color::GREEN,
        _ => Color::White,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Color {
    White,
    Pink,
    Yellow,
    Blue,
    Green,
    Gray,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::White => write!(f, "white"),
            Color::Pink => write!(f, "pink"),
            Color::Yellow => write!(f, "yellow"),
            Color::Blue => write!(f, "blue"),
            Color::Green => write!(f, "green"),
            Color::Gray => write!(f, "gray"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StatusName {
    pub name: String,
    pub color: Color,
}
