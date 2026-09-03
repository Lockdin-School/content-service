use crate::core::quizzes::models::QuizQuestion::QuizQuestion;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewQuiz {
    /// Identifier of the lesson to which this quiz belongs.
    pub lesson_id: Uuid,

    /// Identifier of the topic to which this quiz belongs.
    pub topic_id: Uuid,

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
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Quiz {
    /// Unique identifier for the quiz.
    pub id: Uuid,

    /// Identifier of the lesson to which this quiz belongs.
    pub lesson_id: Uuid,

    /// Identifier of the topic to which this quiz belongs.
    pub topic_id: Uuid,

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

    /// Timestamp at which the quiz was created.
    pub created_at: DateTime<Utc>,

    /// Timestamp at which the quiz was last modified.
    pub updated_at: DateTime<Utc>,
}

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
    pub questions: Vec<QuizQuestion>,

    /// Timestamp at which the quiz was created.
    pub created_at: DateTime<Utc>,

    /// Timestamp at which the quiz was last modified.
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "difficulty", rename_all = "lowercase")]
pub enum QuizDifficulty {
    Easy,
    Medium,
    Hard,
}
