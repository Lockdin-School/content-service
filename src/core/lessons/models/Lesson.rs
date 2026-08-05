use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Lesson {
    // ---------------------------------------------------------------------
    // Identity / Relationship
    // ---------------------------------------------------------------------
    pub id: Uuid,

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

    // ---------------------------------------------------------------------
    // Analytics
    // ---------------------------------------------------------------------
    pub watch_count: i64,

    // ---------------------------------------------------------------------
    // Audit
    // ---------------------------------------------------------------------
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
