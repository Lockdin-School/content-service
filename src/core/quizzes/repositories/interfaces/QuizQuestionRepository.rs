use uuid::Uuid;
use crate::core::quizzes::models::Quiz::Quiz;
use crate::core::quizzes::models::QuizQuestion::QuizQuestion;

#[async_trait::async_trait]
pub trait QuizQuestionRepository {
    async fn create_quiz_question(&self, question: QuizQuestion) -> sqlx::Result<Uuid, sqlx::Error>;
    async fn get_quiz_questions_by_quiz_id(&self, id: Uuid) -> sqlx::Result<Quiz, sqlx::Error>;
}