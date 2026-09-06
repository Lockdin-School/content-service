use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;
/// Represents a new question belonging to a quiz.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct NewQuizQuestion {
    /// Identifier of the quiz to which this question belongs.
    pub quiz_id: Uuid,

    /// Format used to present and evaluate the question.
    pub question_type: QuizQuestionType,

    /// The question presented to the student.
    pub prompt: String,

    /// Number of points awarded for a correct response.
    pub points: i32,

    /// Zero-based position of the question within the quiz.
    pub order: i32,
}

/// Represents a question belonging to a quiz.
///
/// A quiz question defines the prompt presented to the student,
/// the question format, its contribution to the quiz score, and
/// its position within the quiz.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct QuizQuestion {
    /// Unique identifier for the question.
    pub id: Uuid,

    /// Identifier of the quiz to which this question belongs.
    pub quiz_id: Uuid,

    /// Format used to present and evaluate the question.
    pub question_type: QuizQuestionType,

    /// The question presented to the student.
    pub prompt: String,

    /// Number of points awarded for a correct response.
    pub points: i32,

    /// Zero-based position of the question within the quiz.
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type)]
#[serde(rename_all = "camelCase")]
pub struct NewQuizQuestionOption {
    /// Identifier of the quiz question to which this option belongs.
    pub question_id: Uuid,

    /// Text presented to the student as the selectable option.
    pub text: String,

    /// Indicates whether selecting this option constitutes a correct answer.
    pub is_correct: bool,

    /// Zero-based position of the option within the question.
    pub order: i32,
}

/// Represents a selectable option belonging to a quiz question.
///
/// Options are used by question types that require the student
/// to select from a predefined set of answers.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, Eq, Type)]
#[serde(rename_all = "camelCase")]
pub struct QuizQuestionOption {
    /// Unique identifier for the option.
    pub id: Uuid,

    /// Identifier of the quiz question to which this option belongs.
    pub question_id: Uuid,

    /// Text presented to the student as the selectable option.
    pub text: String,

    /// Indicates whether selecting this option constitutes a correct answer.
    pub is_correct: bool,

    /// Zero-based position of the option within the question.
    pub order: i32,
}

/// Defines the supported formats for quiz questions.
#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "question_type", rename_all = "camelCase")]
pub enum QuizQuestionType {
    /// A question with a predefined set of selectable answers.
    MultipleChoice,
}
