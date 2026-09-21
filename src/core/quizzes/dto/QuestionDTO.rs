use crate::core::quizzes::dto::OptionDTO::OptionDTO;
use crate::core::quizzes::models::QuizQuestion::QuizQuestionType;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AggregateQuestion {
    /// Unique identifier for the question.
    pub id: Uuid,
    /// Identifier of the quiz to which this question belongs.
    pub quiz_id: Uuid,
    
    /// Identifier of the concept or lesson being examined.
    pub concept_id: Uuid,
    /// Format used to present and evaluate the question.
    pub question_type: QuizQuestionType,
    /// The question presented to the student.
    pub prompt: String,
    /// Number of points awarded for a correct response.
    pub points: i32,
    /// Zero-based position of the question within the quiz.
    pub order: i32,

    pub options: Vec<OptionDTO>,
}
