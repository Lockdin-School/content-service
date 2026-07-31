use crate::core::materials::models::MaterialStatus::MaterialStatus;
use crate::core::materials::models::MaterialType::MaterialType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateMaterialRequest {
    pub code: Option<String>,
    pub slug: Option<String>,

    pub title: Option<String>,
    pub short_description: Option<String>,
    pub description: Option<String>,

    pub material_type: Option<MaterialType>,

    pub display_order: Option<i32>,
    pub estimated_duration_seconds: Option<i32>,

    pub status: Option<MaterialStatus>,

    pub is_featured: Option<bool>,
    pub is_free: Option<bool>,
}
