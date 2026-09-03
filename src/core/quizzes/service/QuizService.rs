use crate::core::quizzes::models::Quiz::{AggregateQuiz, NewQuiz};
use crate::core::quizzes::quizzes_events::QuizCreatedPayload;
use crate::core::quizzes::repositories::interfaces::QuizQuestionRepository::QuizQuestionRepository;
use crate::core::quizzes::repositories::interfaces::QuizRepository::QuizRepository;
use crate::infrastructure::InternalEventBus::{Event, EventBus};
use crate::utils::code::generate_code;
use crate::utils::slug::generate_slug;
use actix_web::web::Data;
use log::error;
use std::sync::Arc;
use tokio::io;
use uuid::Uuid;

pub struct QuizService {
    repo: Arc<dyn QuizRepository + Send + Sync>,
    questions_repo: Arc<dyn QuizQuestionRepository + Send + Sync>,
    event_bus: Data<EventBus>,
}

impl QuizService {
    pub fn new(
        repo: Arc<dyn QuizRepository + Send + Sync>,
        questions_repo: Arc<dyn QuizQuestionRepository + Send + Sync>,
        event_bus: Data<EventBus>,
    ) -> Self {
        QuizService {
            repo,
            questions_repo,
            event_bus,
        }
    }

    pub async fn create_quiz(&self, new_quiz: NewQuiz) -> Result<Uuid, io::Error> {
        log::info!(target: "QuizService::create_quiz", "create new quiz");
        match self.repo.create_quiz(new_quiz).await {
            Ok(quiz) => {
                let quiz_created_payload = QuizCreatedPayload {
                    quiz_id: quiz.id,
                    code: generate_code("Q"),
                    slug: generate_slug("Q", &quiz.title),
                    title: quiz.title.clone(),
                    description: quiz.description.clone(),
                    topic_id: quiz.topic_id,
                    estimated_duration_seconds: Some(quiz.estimated_duration_minutes),
                };

                if let Err(e) = self
                    .event_bus
                    .send(Event::QuizCreated(quiz_created_payload))
                {
                    log::error!(
                        "quiz.create | internal_event_bus | create_quiz | failed | \"Failed to publish quiz created event\" | {:?}",
                        e
                    );
                };

                Ok(quiz.id)
            }
            Err(e) => {
                error!("Failed to create quiz: {}", e);
                Err(io::Error::other(e))
            }
        }
    }

    pub async fn get_quiz_by_id(&self, id: Uuid) -> Result<AggregateQuiz, io::Error> {
        match self.repo.get_quiz_by_id(id).await {
            Ok(quiz_db) => {
                let quiz_questions = self
                    .questions_repo
                    .get_quiz_questions_by_quiz_id(id)
                    .await
                    .unwrap_or_else(|e| {
                        log::error!("quiz_questions.get | service | get_quiz_questions_by_id | failure | \"Failed to fetch quiz questions for quiz {}: {:?}\" |", id, e);
                        Vec::new()
                });

                let quiz = AggregateQuiz {
                    id: quiz_db.id,
                    lesson_id: quiz_db.lesson_id,
                    title: quiz_db.title,
                    description: quiz_db.description,
                    difficulty: quiz_db.difficulty,
                    passing_score: quiz_db.passing_score,
                    estimated_duration_minutes: quiz_db.estimated_duration_minutes,
                    questions: quiz_questions,
                    created_at: quiz_db.created_at,
                    updated_at: quiz_db.updated_at,
                };

                Ok(quiz)
            }
            Err(error) => {
                log::error!(
                    "quiz.get | service | get_quiz_by_id | failure | \"{:?}\" |",
                    error
                );
                Err(io::Error::other(error.to_string()))
            }
        }
    }
}
