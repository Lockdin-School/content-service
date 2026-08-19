use crate::core::quizzes::models::QuizQuestion::QuizQuestion;
use crate::core::quizzes::models::QuizQuestionResponse::QuizQuestionResponse;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait QuizQuestionResponseRepository {
    async fn save(
        &self,
        question_response: QuizQuestionResponse,
    ) -> sqlx::Result<Uuid, sqlx::Error>;
    async fn get_responses_by_attempt_id(
        &self,
        attempt_id: Uuid,
    ) -> sqlx::Result<Vec<QuizQuestion>, sqlx::Error>;
}
