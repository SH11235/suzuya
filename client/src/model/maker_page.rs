use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MakerState {
    pub id: String,
    pub code_name: String,
    pub is_changed: bool,
}

#[derive(Debug, Serialize)]
pub struct PutMakerRequest {
    pub code_name: String,
}
