use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::core::quizzes::models::QuizQuestion::QuizQuestionOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionDTO {
    /// Unique identifier for the option.
    pub id: Uuid,

    /// Identifier of the quiz question to which this option belongs.
    pub question_id: Uuid,

    /// Text presented to the student as the selectable option.
    pub text: String,

    /// Zero-based position of the option within the question.
    pub order: i32,
}

impl From<QuizQuestionOption> for OptionDTO {
    fn from(option: QuizQuestionOption) -> Self {
        Self {
            id: option.id,
            question_id: option.question_id,
            text: option.text,
            order: option.order,
        }
    }
}