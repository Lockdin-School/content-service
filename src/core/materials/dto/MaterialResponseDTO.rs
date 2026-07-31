use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::core::materials::models::Material::Material;
use crate::core::materials::models::MaterialStatus::MaterialStatus;
use crate::core::materials::models::MaterialType::MaterialType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialResponseDTO {
    pub id: Uuid,

    pub code: String,
    pub slug: String,

    pub title: String,
    pub short_description: Option<String>,
    pub description: Option<String>,

    pub topic_id: Uuid,

    pub material_type: MaterialType,

    pub display_order: i32,
    pub estimated_duration_seconds: Option<i32>,

    pub view_count: i64,

    pub status: MaterialStatus,

    pub is_featured: bool,
    pub is_free: bool,

    pub version: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub published_at: Option<DateTime<Utc>>,
    pub archived_at: Option<DateTime<Utc>>,
}

impl From<Material> for MaterialResponseDTO {
    fn from(material: Material) -> Self {
        Self {
            id: material.id,
            code: material.code,
            slug: material.slug,
            title: material.title,
            short_description: material.short_description,
            description: material.description,
            topic_id: material.topic_id,
            material_type: material.material_type,
            display_order: material.display_order,
            estimated_duration_seconds: material.estimated_duration_seconds,
            view_count: material.view_count,
            status: material.status,
            is_featured: material.is_featured,
            is_free: material.is_free,
            version: material.version,
            created_at: material.created_at,
            updated_at: material.updated_at,
            published_at: material.published_at,
            archived_at: material.archived_at,
        }
    }
}

