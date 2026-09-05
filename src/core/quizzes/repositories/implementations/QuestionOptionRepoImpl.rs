use crate::core::quizzes::models::QuizQuestion::{NewQuizQuestionOption, QuizQuestionOption};
use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::core::quizzes::repositories::interfaces::option::QuestionOptionRepository::QuestionOptionRepository;

pub struct PostgresQuestionOptionRepo {
    pool: PgPool,
}

#[async_trait::async_trait]
impl QuestionOptionRepository for PostgresQuestionOptionRepo {
    async fn create_option(
        &self,
        new_option: NewQuizQuestionOption,
    ) -> sqlx::Result<Uuid, Error> {
        let new_id = Uuid::now_v7();
        let (id,): (Uuid,) = sqlx::query_as(
            "INSERT INTO quiz_question_options(id, question_id, text, is_correct, \"order\") \
         VALUES ($1, $2, $3, $4, $5) RETURNING id",
        )
            .bind(new_id)
            .bind(new_option.question_id)
            .bind(new_option.text)
            .bind(new_option.is_correct)
            .bind(new_option.order)
            .fetch_one(&self.pool)
            .await?;

        Ok(id)
    }

    async fn get_options_by_question_id(
        &self,
        question_id: Uuid,
    ) -> sqlx::Result<Vec<QuizQuestionOption>, Error> {
        let options = sqlx::query_as(
            "SELECT * FROM quiz_question_options WHERE question_id = $1 ORDER BY \"order\"",
        )
            .bind(question_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(options)
    }

    async fn get_option_by_id(&self, id: Uuid) -> sqlx::Result<Option<QuizQuestionOption>, Error> {
        let option = sqlx::query_as("SELECT * FROM quiz_question_options WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(option)
    }
}

impl PostgresQuestionOptionRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}