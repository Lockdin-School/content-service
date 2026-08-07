use serde::Serialize;
use uuid::Uuid;
use crate::core::lessons::models::Lesson::Lesson;

#[derive(Debug, Clone, Serialize)]
pub struct ReadLessonResponse {
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

impl From<Lesson> for ReadLessonResponse {
    fn from(lesson: Lesson) -> Self {
        Self {
            id: lesson.id,
            video_url: lesson.video_url,
            thumbnail_url: lesson.thumbnail_url,
            transcript_url: lesson.transcript_url,
            duration_seconds: lesson.duration_seconds,
            resolution: lesson.resolution,
            language: lesson.language,
            captions_url: lesson.captions_url,
            watch_count: lesson.watch_count,
            created_at: lesson.created_at,
            updated_at: lesson.updated_at,
        }
    }
}