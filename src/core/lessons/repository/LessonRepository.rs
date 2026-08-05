use crate::core::lessons::dto::CreateLessonRequest::CreateLessonRequest;
use crate::core::lessons::models::Lesson::Lesson;

#[async_trait::async_trait]
pub trait LessonRepository {
    async fn get_lesson_by_id(
        &self,
        material_id: uuid::Uuid
    ) -> sqlx::Result<Option<Lesson>, sqlx::Error>;

    async fn create_lesson(
        &self,
        lesson: &CreateLessonRequest
    ) -> sqlx::Result<Lesson, sqlx::Error>;
}