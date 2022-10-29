use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkerState {
    pub id: String,
    pub name: String,
    pub is_changed: bool,
    pub is_saved: bool,
}

#[derive(Debug, Serialize)]
pub struct PutWorkerRequest {
    pub name: String,
}
