use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub title: String,
    pub description: Option<String>,
    pub expression: Option<String>,
    pub steps: Vec<String>,
    pub conclusion: Option<String>,
}
