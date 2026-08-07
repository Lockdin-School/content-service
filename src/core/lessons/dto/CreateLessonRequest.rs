use serde::Deserialize;
use uuid::Uuid;
use crate::core::materials::models::MaterialType::MaterialType;

#[derive(Debug, Clone, Deserialize)]
pub struct IncomingCreateLessonRequest {
    // ---------------------------------------------------------------------
    // Material Properties // Combined On Purpose
    // ---------------------------------------------------------------------
    pub title: String,
    pub short_description: Option<String>,
    pub description: Option<String>,

    pub topic_id: Uuid,

    pub material_type: MaterialType,

    // ---------------------------------------------------------------------
    // Video
    // ---------------------------------------------------------------------
    pub video_url: String,
    pub thumbnail_url: Option<String>,
    pub transcript_url: Option<String>,

    // ---------------------------------------------------------------------
    // Playback
    // ---------------------------------------------------------------------
    pub duration_seconds: i32,
    pub resolution: Option<String>,
    pub language: Option<String>,
    pub captions_url: Option<String>,

    pub is_featured: bool,
    pub is_free: bool,
}

pub struct CreateLessonRequest {
    // ---------------------------------------------------------------------
    // Video
    // ---------------------------------------------------------------------
    pub video_url: String,
    pub thumbnail_url: Option<String>,
    pub transcript_url: Option<String>,

    // ---------------------------------------------------------------------
    // Playback
    // ---------------------------------------------------------------------
    pub duration_seconds: i32,
    pub resolution: Option<String>,
    pub language: Option<String>,
    pub captions_url: Option<String>,
}