use crate::core::quizzes::models::QuizAttempt::{QuizAttempt, QuizAttemptNew};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait QuizAttemptRepository {
    async fn save(&self, attempt: QuizAttemptNew) -> sqlx::Result<Uuid, sqlx::Error>;

    async fn get_quiz_attempt_by_student_id(
        &self,
        quiz_id: Uuid,
        student_id: Uuid,
    ) -> sqlx::Result<QuizAttempt, sqlx::Error>;

    async fn get_quiz_attempt_by_id(&self, id: Uuid) -> sqlx::Result<QuizAttempt, sqlx::Error>;

    async fn update_attempt(
        &self,
        id: Uuid,
        attempt: QuizAttemptNew,
    ) -> sqlx::Result<Option<QuizAttempt>, sqlx::Error>;
}
