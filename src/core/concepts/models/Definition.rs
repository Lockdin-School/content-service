use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    pub title: Option<String>,
    pub text: String,
}
