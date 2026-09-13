use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub event_type: String, // snake_case
    pub version: i32,

    pub student_id: Uuid,

    pub occurred_at: DateTime<Utc>,

    pub source_service: String,
    pub source_event_id: Uuid,

    pub data: serde_json::Value,
}
