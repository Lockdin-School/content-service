use crate::core::quizzes::models::QuizAttempt::{QuizAttempt, QuizAttemptNew, QuizAttemptRow};
use crate::core::quizzes::repositories::interfaces::attempt::QuizAttemptRepository::QuizAttemptRepository;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct PostgresQuizAttemptRepo {
    pool: PgPool,
}

#[async_trait::async_trait]
impl QuizAttemptRepository for PostgresQuizAttemptRepo {
    async fn save(&self, attempt: QuizAttemptNew) -> sqlx::Result<Uuid, Error> {
        let new_id = Uuid::now_v7();

        let (id,): (Uuid,) = sqlx::query_as(
            "INSERT INTO quiz_attempts(
                id,
                quiz_id,
                student_id,
                status,
                started_at,
                ended_at,
                score,
                percentage
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (quiz_id, student_id)
            WHERE status = 'inprogress'
            DO UPDATE SET
                updated_at = NOW()
            RETURNING id",
        )
        .bind(new_id)
        .bind(attempt.quiz_id)
        .bind(attempt.student_id)
        .bind(attempt.status)
        .bind(attempt.started_at)
        .bind(attempt.ended_at)
        .bind(attempt.score)
        .bind(attempt.percentage)
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }

    async fn get_quiz_attempt_by_student_id(
        &self,
        quiz_id: Uuid,
        student_id: Uuid,
    ) -> sqlx::Result<QuizAttempt, Error> {
        let attempt: QuizAttemptRow = sqlx::query_as(
            "SELECT
                id,
                quiz_id,
                student_id,
                status,
                started_at,
                ended_at,
                score,
                percentage,
                created_at,
                updated_at
            FROM quiz_attempts
            WHERE quiz_id = $1
            AND student_id = $2
            AND status = 'inprogress'
            ORDER BY started_at DESC
            LIMIT 1",
        )
        .bind(quiz_id)
        .bind(student_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(attempt.into())
    }

    async fn get_quiz_attempt_by_id(&self, id: Uuid) -> sqlx::Result<QuizAttempt, Error> {
        let attempt: QuizAttemptRow = sqlx::query_as(
            "SELECT
                id,
                quiz_id,
                student_id,
                status,
                started_at,
                ended_at,
                score,
                percentage,
                created_at,
                updated_at
            FROM quiz_attempts
            WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(attempt.into())
    }

    async fn update_attempt(
        &self,
        id: Uuid,
        attempt: QuizAttemptNew,
    ) -> sqlx::Result<Option<QuizAttempt>, Error> {
        let updated_at = chrono::Utc::now();
        let updated_attempt: Option<QuizAttemptRow> = sqlx::query_as(
            "UPDATE quiz_attempts
            SET
                quiz_id = $2,
                student_id = $3,
                status = $4,
                started_at = $5,
                ended_at = $6,
                score = $7,
                percentage = $8,
                updated_at = $9
            WHERE id = $1
            RETURNING
                id,
                quiz_id,
                student_id,
                status,
                started_at,
                ended_at,
                score,
                percentage,
                created_at,
                updated_at",
        )
        .bind(id)
        .bind(attempt.quiz_id)
        .bind(attempt.student_id)
        .bind(attempt.status)
        .bind(attempt.started_at)
        .bind(attempt.ended_at)
        .bind(attempt.score)
        .bind(attempt.percentage)
        .bind(updated_at)
        .fetch_optional(&self.pool)
        .await?;

        Ok(updated_attempt.map(QuizAttempt::from))
    }
}

impl PostgresQuizAttemptRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
