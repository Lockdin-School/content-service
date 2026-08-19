use crate::core::quizzes::models::QuizAttempt::QuizAttemptDB;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait QuizAttemptRepository {
    async fn save(&self, attempt: QuizAttemptDB) -> sqlx::Result<Uuid, sqlx::Error>;
    async fn get_attempt_by_student_id(
        &self,
        quiz_id: Uuid,
        student_id: Uuid,
    ) -> sqlx::Result<QuizAttemptDB, sqlx::Error>;
}
