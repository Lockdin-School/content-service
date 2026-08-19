use sqlx::{PgPool};

pub struct QuizAttemptRepoImpl {
    pool: PgPool
}
