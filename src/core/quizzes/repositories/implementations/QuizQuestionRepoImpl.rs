use crate::core::quizzes::models::QuizQuestion::{NewQuizQuestion, QuizQuestion};
use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::core::quizzes::repositories::interfaces::question::QuizQuestionRepository::QuizQuestionRepository;

pub struct PostgresQuizQuestionRepo {
    pool: PgPool,
}

#[async_trait::async_trait]
impl QuizQuestionRepository for PostgresQuizQuestionRepo {
    async fn create_quiz_question(&self, question: NewQuizQuestion) -> sqlx::Result<Uuid, Error> {
        let new_id = Uuid::now_v7();
        let (id,): (Uuid,) = sqlx::query_as(
            "INSERT INTO quiz_questions(id, quiz_id, question_type, prompt, points, \"order\") \
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
        )
            .bind(new_id)
            .bind(question.quiz_id)
            .bind(question.question_type)
            .bind(question.prompt)
            .bind(question.points)
            .bind(question.order)
            .fetch_one(&self.pool)
            .await?;

        Ok(id)
    }

    async fn get_quiz_questions_by_quiz_id(
        &self,
        id: Uuid,
    ) -> sqlx::Result<Vec<QuizQuestion>, Error> {
        let questions =
            sqlx::query_as("SELECT * FROM quiz_questions WHERE quiz_id = $1 ORDER BY \"order\"")
                .bind(id)
                .fetch_all(&self.pool)
                .await?;

        Ok(questions)
    }

    async fn get_quiz_question_by_id(&self, id: Uuid) -> sqlx::Result<Option<QuizQuestion>, Error> {
        let question = sqlx::query_as("SELECT * FROM quiz_questions WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        
        Ok(question)
    }
}

impl PostgresQuizQuestionRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
