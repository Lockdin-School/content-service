use crate::core::quizzes::models::QuizQuestion::{NewQuizQuestionOption, QuizQuestionOption};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait QuestionOptionRepository {
    async fn create_option(
        &self,
        new_option: NewQuizQuestionOption,
    ) -> sqlx::Result<Uuid, sqlx::Error>;
    async fn get_options_by_question_id(
        &self,
        question_id: Uuid,
    ) -> sqlx::Result<Vec<QuizQuestionOption>, sqlx::Error>;
    async fn get_option_by_id(
        &self,
        id: Uuid,
    ) -> sqlx::Result<Option<QuizQuestionOption>, sqlx::Error>;
}
