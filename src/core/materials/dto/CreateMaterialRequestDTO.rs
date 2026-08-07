use crate::core::materials::models::MaterialType::MaterialType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMaterialRequest {
    pub code: String,
    pub slug: String,

    pub title: String,
    pub short_description: Option<String>,
    pub description: Option<String>,

    pub topic_id: Uuid,

    pub material_type: MaterialType,

    pub estimated_duration_seconds: Option<i32>,

    pub is_featured: bool,
    pub is_free: bool,
}
