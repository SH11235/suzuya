use serde::{Deserialize, Serialize};

// entity
#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ItemModel {
    pub id: String,
    pub name: String,
    pub product_code: Option<String>,
    pub sku: Option<i32>, // 種類数
    pub illust_status: String,
    // pic：person in charge
    pub pic_illust_id: Option<String>, // from user 「イラスト担当者」
    pub design_status: String,
    pub pic_design_id: Option<String>, // from user 「デザイン担当者」
    pub maker_id: Option<String>,      // from maker
    pub retail_price: Option<i32>,     // 上代
    pub resubmission: bool,
    pub line: String,
    // double_check_person_id: Option<String>, // from user 「二重チェック者」
    pub rough_coordinator_id: Option<String>, // from user 「ラフ担当者」
    pub rough_check_person_id: Option<String>, // from user 「ラフチェック者」
    pub line_art_coordinator_id: Option<String>, // from user 「線画担当者」
    pub line_art_check_person_id: Option<String>, // from user 「線画チェック者」
    pub coloring_coordinator_id: Option<String>, // from user 「彩色担当者」
    pub coloring_check_person_id: Option<String>, // from user 「彩色チェック者」
    pub design_coordinator_id: Option<String>, // from user 「デザイン担当者」
    pub design_check_person_id: Option<String>, // from user 「デザインチェック者」
    pub submission_data_coordinator_id: Option<String>, // from user 「入稿データ担当者」
    pub submission_data_check_person_id: Option<String>, // from user 「入稿データチェック者」
    pub announcement_materials_coordinator_id: Option<String>, // from user 「告知物担当者」
    pub announcement_materials_check_person_id: Option<String>, // from user 「告知物チェック者」
    pub jan_coordinator_id: Option<String>, // from user 「JAN担当者」
    pub jan_check_person_id: Option<String>, // from user 「JANチェック者」
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkerModel {
    pub id: String,
    pub name: String,
    pub deleted: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MakerModel {
    pub id: String,
    pub code_name: String,
    pub deleted: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NameOptionIdPair {
    pub name: String,
    pub id: Option<String>,
}
