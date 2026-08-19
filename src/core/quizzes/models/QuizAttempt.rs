use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;
use crate::core::quizzes::models::QuizQuestionResponse::QuizQuestionResponse;

/// Represents a student's attempt to complete a quiz.
///
/// An attempt captures a single instance of a student engaging with
/// a quiz, including its lifecycle, timing, and resulting score.
#[derive(Debug, Clone, FromRow)]
pub struct QuizAttemptDB {
    /// Unique identifier for the attempt.
    pub id: Uuid,

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
    pub percentage: Option<f32>,

    /// Timestamp at which the attempt record was created.
    pub created_at: DateTime<Utc>,

    /// Timestamp at which the attempt record was last modified.
    pub updated_at: DateTime<Utc>,
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
    pub percentage: Option<f32>,

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

