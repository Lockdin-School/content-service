use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Misconception {
    pub misconception: String,
    pub correction: String,
}
