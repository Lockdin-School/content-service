use crate::core::quizzes::models::QuizQuestionResponse::{
    EvaluatedQuizQuestionResponseNew, QuizQuestionResponse,
};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait QuizQuestionResponseRepository {
    async fn save(
        &self,
        question_response: EvaluatedQuizQuestionResponseNew,
    ) -> sqlx::Result<Uuid, sqlx::Error>;

    async fn get_responses_by_attempt_id(
        &self,
        attempt_id: Uuid,
    ) -> sqlx::Result<Vec<QuizQuestionResponse>, sqlx::Error>;

    async fn update_response(
        &self,
        id: Uuid,
        question_response: EvaluatedQuizQuestionResponseNew,
    ) -> sqlx::Result<Option<QuizQuestionResponse>, sqlx::Error>;
}
