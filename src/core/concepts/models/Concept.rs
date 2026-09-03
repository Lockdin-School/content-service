use crate::core::concepts::models::Definition::Definition;
use crate::core::concepts::models::Example::Example;
use crate::core::concepts::models::ExplanationSection::ExplanationSection;
use crate::core::concepts::models::Misconception::Misconception;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Concept {
    pub id: Uuid,
    pub topic_id: Uuid,

    pub title: String,
    pub display_order: i32,

    pub definitions: Vec<Definition>,
    pub explanation: Vec<ExplanationSection>,

    pub analogy: Option<Vec<ExplanationSection>>,

    pub examples: Vec<Example>,
    pub misconceptions: Vec<Misconception>,

    pub summary: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptNew {
    pub topic_id: Uuid,

    pub title: String,
    pub display_order: i32,

    pub definitions: Vec<Definition>,
    pub explanation: Vec<ExplanationSection>,

    pub analogy: Option<Vec<ExplanationSection>>,

    pub examples: Vec<Example>,
    pub misconceptions: Vec<Misconception>,

    pub summary: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptUpdate {
    pub topic_id: Option<Uuid>,

    pub title: Option<String>,
    pub display_order: Option<i32>,

    pub definitions: Option<Vec<Definition>>,
    pub explanation: Option<Vec<ExplanationSection>>,

    pub analogy: Option<Option<Vec<ExplanationSection>>>,

    pub examples: Option<Vec<Example>>,
    pub misconceptions: Option<Vec<Misconception>>,

    pub summary: Option<Option<String>>,
}
