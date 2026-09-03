use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct QuizCreatedPayload {
    pub quiz_id: Uuid,
    pub code: String,
    pub slug: String,

    pub title: String,
    pub description: Option<String>,

    pub topic_id: Uuid,

    pub estimated_duration_seconds: Option<i32>,
}
