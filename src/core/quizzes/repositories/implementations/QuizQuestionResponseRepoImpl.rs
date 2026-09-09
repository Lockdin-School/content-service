use crate::core::quizzes::models::QuizQuestionResponse::{
    EvaluatedQuizQuestionResponseNew, QuizQuestionResponse,
};
use crate::core::quizzes::repositories::interfaces::response::QuizQuestionResponseRepository::QuizQuestionResponseRepository;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct PostgresQuizQuestionResponseRepo {
    pool: PgPool,
}

#[async_trait::async_trait]
impl QuizQuestionResponseRepository for PostgresQuizQuestionResponseRepo {
    async fn save(
        &self,
        question_response: EvaluatedQuizQuestionResponseNew,
    ) -> sqlx::Result<Uuid, Error> {
        let new_id = Uuid::now_v7();

        let (id,): (Uuid,) = sqlx::query_as(
            "INSERT INTO quiz_question_responses(
                id,
                attempt_id,
                question_id,
                selected_option_id,
                is_correct,
                answered_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id",
        )
        .bind(new_id)
        .bind(question_response.attempt_id)
        .bind(question_response.question_id)
        .bind(question_response.selected_option_id)
        .bind(question_response.is_correct)
        .bind(question_response.answered_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }

    async fn get_responses_by_attempt_id(
        &self,
        attempt_id: Uuid,
    ) -> sqlx::Result<Vec<QuizQuestionResponse>, Error> {
        let responses = sqlx::query_as(
            "SELECT
                id,
                attempt_id,
                question_id,
                selected_option_id,
                is_correct,
                answered_at
            FROM quiz_question_responses
            WHERE attempt_id = $1
            ORDER BY answered_at",
        )
        .bind(attempt_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(responses)
    }

    async fn update_response(
        &self,
        id: Uuid,
        question_response: EvaluatedQuizQuestionResponseNew,
    ) -> sqlx::Result<Option<QuizQuestionResponse>, Error> {
        let updated_response = sqlx::query_as(
            "UPDATE quiz_question_responses
            SET
                attempt_id = $2,
                question_id = $3,
                selected_option_id = $4,
                is_correct = $5,
                answered_at = $6
            WHERE id = $1
            RETURNING
                id,
                attempt_id,
                question_id,
                selected_option_id,
                is_correct,
                answered_at",
        )
        .bind(id)
        .bind(question_response.attempt_id)
        .bind(question_response.question_id)
        .bind(question_response.selected_option_id)
        .bind(question_response.is_correct)
        .bind(question_response.answered_at)
        .fetch_optional(&self.pool)
        .await?;

        Ok(updated_response)
    }
}

impl PostgresQuizQuestionResponseRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
