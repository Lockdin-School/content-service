use crate::core::quizzes::models::Quiz::{NewQuiz, Quiz};
use crate::core::quizzes::repositories::interfaces::QuizRepository::QuizRepository;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct PostgresQuizRepo {
    pool: PgPool,
}

#[async_trait::async_trait]
impl QuizRepository for PostgresQuizRepo {
    async fn create_quiz(&self, quiz: NewQuiz) -> sqlx::Result<Quiz, Error> {
        let id = Uuid::now_v7();
        let quiz = sqlx::query_as("INSERT INTO quizzes(id, lesson_id, topic_id, title, description, difficulty, passing_score, estimated_duration_minutes) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *")
            .bind(id)
            .bind(quiz.lesson_id)
            .bind(quiz.topic_id)
            .bind(quiz.title)
            .bind(quiz.description)
            .bind(quiz.difficulty)
            .bind(quiz.passing_score)
            .bind(quiz.estimated_duration_minutes)
            .fetch_one(&self.pool)
            .await?;

        Ok(quiz)
    }

    async fn get_quiz_by_id(&self, id: Uuid) -> sqlx::Result<Quiz, Error> {
        let quiz = sqlx::query_as("SELECT * FROM quizzes WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(quiz)
    }
}

impl PostgresQuizRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
