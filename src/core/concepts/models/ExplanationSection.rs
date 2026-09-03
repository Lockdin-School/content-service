use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationSection {
    pub subtitle: Option<String>,
    pub paragraphs: Vec<String>,
}
