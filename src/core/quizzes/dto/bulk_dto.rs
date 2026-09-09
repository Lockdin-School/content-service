use crate::core::quizzes::models::Quiz::NewQuiz;
use crate::core::quizzes::models::QuizQuestion::QuizQuestionType;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkQuizUpload {
    pub quiz: NewQuiz,
    pub questions: Vec<BulkQuizQuestionUpload>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkQuizQuestionUpload {
    pub question_type: QuizQuestionType,
    pub prompt: String,
    pub points: i32,
    pub order: i32,
    pub options: Vec<BulkQuizQuestionOptionUpload>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkQuizQuestionOptionUpload {
    pub text: String,
    pub is_correct: bool,
    pub order: i32,
}
