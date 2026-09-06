use crate::core::quizzes::dto::QuestionDTO::AggregateQuestion;
use crate::core::quizzes::models::Quiz::QuizDifficulty;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a quiz associated with a lesson.
///
/// A quiz is a structured assessment used to evaluate a student's
/// understanding of the material covered by its associated lesson.
/// It owns the questions that make up the assessment and defines
/// the conditions and metadata under which the quiz is presented.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AggregateQuiz {
    /// Unique identifier for the quiz.
    pub id: Uuid,

    /// Identifier of the lesson to which this quiz belongs.
    pub lesson_id: Uuid,

    /// Title displayed to students.
    pub title: String,

    /// Optional description providing additional context about the quiz.
    pub description: Option<String>,

    /// Difficulty level assigned to the quiz.
    pub difficulty: QuizDifficulty,

    /// Minimum score required to pass the quiz.
    pub passing_score: i32,

    /// Expected time, in minutes, required to complete the quiz.
    pub estimated_duration_minutes: i32,

    /// Questions belonging to this quiz, maintained in their defined order.
    pub questions: Vec<AggregateQuestion>,

    /// Timestamp at which the quiz was created.
    pub created_at: DateTime<Utc>,

    /// Timestamp at which the quiz was last modified.
    pub updated_at: DateTime<Utc>,
}
