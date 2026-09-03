use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a student's response to a question during a quiz attempt.
///
/// A response records the answer submitted by the student and the
/// resulting evaluation for that individual question.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizQuestionResponse {
    /// Unique identifier for the response.
    pub id: Uuid,

    /// Identifier of the quiz attempt containing this response.
    pub attempt_id: Uuid,

    /// Identifier of the question being answered.
    pub question_id: Uuid,

    /// Identifier of the option selected by the student.
    ///
    /// This is optional because not every response necessarily contains
    /// a selected option, particularly for unanswered questions or
    /// question types that do not use predefined options.
    pub selected_option_id: Option<Uuid>,

    /// Indicates whether the submitted response was evaluated as correct.
    pub is_correct: bool,

    /// Timestamp at which the response was submitted.
    pub answered_at: DateTime<Utc>,
}
