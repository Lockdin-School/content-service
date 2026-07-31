use crate::core::materials::models::MaterialStatus::MaterialStatus;
use crate::core::materials::models::MaterialType::MaterialType;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Material {
    // ---------------------------------------------------------------------
    // Identity
    // ---------------------------------------------------------------------
    pub id: Uuid,
    pub code: String,
    pub slug: String,
    pub title: String,
    pub short_description: Option<String>,
    pub description: Option<String>,

    //------------------------------------------------------------------------
    // Relationships
    //------------------------------------------------------------------------
    pub topic_id: Uuid,

    // ---------------------------------------------------------------------
    // Classification
    // ---------------------------------------------------------------------
    pub material_type: MaterialType,

    // ---------------------------------------------------------------------
    // Presentation
    // ---------------------------------------------------------------------
    pub display_order: i32,
    pub estimated_duration_seconds: Option<i32>,

    // ---------------------------------------------------------------------
    // Cached Statistics
    // ---------------------------------------------------------------------
    pub view_count: i64,

    // ---------------------------------------------------------------------
    // Status
    // ---------------------------------------------------------------------
    pub status: MaterialStatus,
    pub is_featured: bool,
    pub is_free: bool,

    // ---------------------------------------------------------------------
    // Versioning
    // ---------------------------------------------------------------------
    pub version: i32,

    // ---------------------------------------------------------------------
    // Audit
    // ---------------------------------------------------------------------
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}
