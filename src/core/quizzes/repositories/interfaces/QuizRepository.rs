use crate::core::quizzes::models::Quiz::{NewQuiz, Quiz};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait QuizRepository {
    async fn create_quiz(&self, quiz: NewQuiz) -> sqlx::Result<Quiz, sqlx::Error>;
    async fn get_quiz_by_id(&self, id: Uuid) -> sqlx::Result<Quiz, sqlx::Error>;
}
