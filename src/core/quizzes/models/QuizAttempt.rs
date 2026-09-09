use crate::core::quizzes::models::QuizQuestionResponse::QuizQuestionResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

/// Represents a student's attempt to complete a quiz.
///
/// An attempt captures a single instance of a student engaging with
/// a quiz, including its lifecycle, timing, and resulting score.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizAttemptNew {
    /// Identifier of the quiz being attempted.
    pub quiz_id: Uuid,

    /// Identifier of the student completing the attempt.
    pub student_id: Uuid,

    /// Current lifecycle state of the attempt.
    pub status: QuizAttemptStatus,

    /// Timestamp at which the student started the attempt.
    pub started_at: DateTime<Utc>,

    /// Timestamp at which the attempt was completed or abandoned.
    pub ended_at: Option<DateTime<Utc>>,

    /// Total points earned during the attempt.
    pub score: Option<i32>,

    /// Percentage score achieved during the attempt.
    pub percentage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizAttempt {
    /// Unique identifier for the attempt.
    pub id: Uuid,

    /// Identifier of the quiz being attempted.
    pub quiz_id: Uuid,

    /// Identifier of the student completing the attempt.
    pub student_id: Uuid,

    /// Current lifecycle state of the attempt.
    pub status: QuizAttemptStatus,

    /// Responses belonging to this attempt.
    pub responses: Vec<QuizQuestionResponse>,

    /// Timestamp at which the student started the attempt.
    pub started_at: DateTime<Utc>,

    /// Timestamp at which the attempt was completed or abandoned.
    pub ended_at: Option<DateTime<Utc>>,

    /// Total points earned during the attempt.
    pub score: Option<i32>,

    /// Percentage score achieved during the attempt.
    pub percentage: Option<f64>,

    /// Timestamp at which the attempt record was created.
    pub created_at: DateTime<Utc>,

    /// Timestamp at which the attempt record was last modified.
    pub updated_at: DateTime<Utc>,
}

/// Defines the lifecycle states of a quiz attempt.
#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "status", rename_all = "lowercase")]
pub enum QuizAttemptStatus {
    /// The student has started the quiz but has not yet completed it.
    InProgress,

    /// The student has submitted the quiz, and the attempt is complete.
    Completed,

    /// The student left the quiz before submitting it.
    Abandoned,
}

#[derive(Debug, Clone, FromRow)]
pub struct QuizAttemptRow {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub student_id: Uuid,
    pub status: QuizAttemptStatus,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub score: Option<i32>,
    pub percentage: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<QuizAttemptRow> for QuizAttempt {
    fn from(row: QuizAttemptRow) -> Self {
        Self {
            id: row.id,
            quiz_id: row.quiz_id,
            student_id: row.student_id,
            status: row.status,
            responses: Vec::new(),
            started_at: row.started_at,
            ended_at: row.ended_at,
            score: row.score,
            percentage: row.percentage,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
