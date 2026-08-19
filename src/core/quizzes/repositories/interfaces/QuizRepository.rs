use uuid::Uuid;
use crate::core::quizzes::models::Quiz::Quiz;

#[async_trait::async_trait]
pub trait QuizRepository {
    async fn create_quiz(&self, quiz: Quiz) -> sqlx::Result<Uuid, sqlx::Error>;
    async fn get_quiz_by_id(&self, id: Uuid) -> sqlx::Result<Quiz, sqlx::Error>;
}