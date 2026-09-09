use crate::core::quizzes::models::QuizAttempt::{QuizAttempt, QuizAttemptStatus};
use crate::core::quizzes::models::QuizQuestionResponse::QuizQuestionResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateQuizAttempt {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub student_id: Uuid,
    pub status: QuizAttemptStatus,
    pub responses: Vec<QuizQuestionResponse>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub score: Option<i32>,
    pub percentage: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AggregateQuizAttempt {
    pub fn from_attempt_and_responses(
        attempt: QuizAttempt,
        responses: Vec<QuizQuestionResponse>,
    ) -> Self {
        Self {
            id: attempt.id,
            quiz_id: attempt.quiz_id,
            student_id: attempt.student_id,
            status: attempt.status,
            responses,
            started_at: attempt.started_at,
            ended_at: attempt.ended_at,
            score: attempt.score,
            percentage: attempt.percentage,
            created_at: attempt.created_at,
            updated_at: attempt.updated_at,
        }
    }
}
