use crate::core::quizzes::models::QuizQuestion::{NewQuizQuestion, QuizQuestion};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait QuizQuestionRepository {
    async fn create_quiz_question(
        &self,
        question: NewQuizQuestion,
    ) -> sqlx::Result<Uuid, sqlx::Error>;
    async fn get_quiz_questions_by_quiz_id(
        &self,
        id: Uuid,
    ) -> sqlx::Result<Vec<QuizQuestion>, sqlx::Error>;
}
