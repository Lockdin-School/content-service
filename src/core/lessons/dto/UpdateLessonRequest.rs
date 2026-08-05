use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateLessonRequest {
    // ---------------------------------------------------------------------
    // Video
    // ---------------------------------------------------------------------
    pub video_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub transcript_url: Option<String>,

    // ---------------------------------------------------------------------
    // Playback
    // ---------------------------------------------------------------------
    pub duration_seconds: Option<i32>,
    pub resolution: Option<String>,
    pub language: Option<String>,
    pub captions_url: Option<String>,
}