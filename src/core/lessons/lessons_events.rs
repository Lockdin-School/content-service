use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LessonCreatedPayload {
    pub material_id: Uuid,
    pub code: String,
    pub slug: String,

    pub title: String,
    pub short_description: Option<String>,
    pub description: Option<String>,

    pub topic_id: Uuid,

    pub estimated_duration_seconds: Option<i32>,

    pub is_featured: bool,
    pub is_free: bool,
}