use crate::core::quizzes::dto::OptionDTO::OptionDTO;
use crate::core::quizzes::dto::QuestionDTO::AggregateQuestion;
use crate::core::quizzes::dto::QuizDTO::AggregateQuiz;
use crate::core::quizzes::models::Quiz::NewQuiz;
use crate::core::quizzes::models::QuizQuestion::{
    NewQuizQuestion, NewQuizQuestionOption, QuizQuestionOption,
};
use crate::core::quizzes::quizzes_events::QuizCreatedPayload;
use crate::core::quizzes::repositories::interfaces::option::QuestionOptionRepository::QuestionOptionRepository;
use crate::core::quizzes::repositories::interfaces::question::QuizQuestionRepository::QuizQuestionRepository;
use crate::core::quizzes::repositories::interfaces::quiz::QuizRepository::QuizRepository;
use crate::infrastructure::InternalEventBus::{Event, EventBus};
use crate::utils::code::generate_code;
use crate::utils::slug::generate_slug;
use actix_web::web::Data;
use log::error;
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use tokio::io;
use uuid::Uuid;

pub struct QuizService {
    repo: Arc<dyn QuizRepository + Send + Sync>,
    questions_repo: Arc<dyn QuizQuestionRepository + Send + Sync>,
    option_repo: Arc<dyn QuestionOptionRepository + Send + Sync>,
    event_bus: Data<EventBus>,
}

impl QuizService {
    pub fn new(
        repo: Arc<dyn QuizRepository + Send + Sync>,
        questions_repo: Arc<dyn QuizQuestionRepository + Send + Sync>,
        option_repo: Arc<dyn QuestionOptionRepository + Send + Sync>,
        event_bus: Data<EventBus>,
    ) -> Self {
        QuizService {
            repo,
            questions_repo,
            option_repo,
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
                    .expect("Failed to fetch quiz questions");

                let mut aggregate_questions: Vec<AggregateQuestion> = Vec::new();
                for question in quiz_questions.into_iter() {
                    let q = self
                        .get_question_by_id(question.id)
                        .await
                        .expect("Failed to fetch question");
                    aggregate_questions.push(q);
                }

                // Assembling the quiz
                let quiz = AggregateQuiz {
                    id: quiz_db.id,
                    lesson_id: quiz_db.lesson_id,
                    title: quiz_db.title,
                    description: quiz_db.description,
                    difficulty: quiz_db.difficulty,
                    passing_score: quiz_db.passing_score,
                    estimated_duration_minutes: quiz_db.estimated_duration_minutes,
                    questions: aggregate_questions,
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

    // Questions
    pub async fn create_question(&self, new_question: NewQuizQuestion) -> Result<Uuid, io::Error> {
        log::info!(
            "question.create | service | create_question | started | \"Creating question.\" | quiz_id={}",
            new_question.quiz_id
        );

        match self.questions_repo.create_quiz_question(new_question).await {
            Ok(id) => {
                log::info!(
                    "question.create | service | create_question | success | \"Created question successfully.\" | question_id={}",
                    id
                );

                Ok(id)
            }
            Err(error) => {
                log::error!(
                    "question.create | service | create_question | failure | \"Failed creating question.\" | error=\"{}\"",
                    error
                );

                Err(io::Error::other(error.to_string()))
            }
        }
    }

    pub async fn get_question_by_id(&self, id: Uuid) -> Result<AggregateQuestion, io::Error> {
        log::info!(
            "question.get | service | get_quiz_question_by_id | started | \"Getting question.\" | question_id={}",
            id
        );

        match self.questions_repo.get_quiz_question_by_id(id).await {
            Ok(question) => {
                log::info!(
                    "question.get | service | get_quiz_question_by_id | progressing | \"Returning the option\" | question_id={}",
                    id
                );

                match question {
                    Some(question) => {
                        log::info!(
                            "question.get | service | get_quiz_question_by_id | success | \"Successfully returned the quiz question\" | question_id={}",
                            id
                        );
                        let options = self
                            .option_repo
                            .get_options_by_question_id(question.id)
                            .await
                            .expect("Failed to fetch options for question")
                            .into_iter()
                            .map(OptionDTO::from)
                            .collect();

                        let question = AggregateQuestion {
                            id,
                            quiz_id: question.quiz_id,
                            question_type: question.question_type,
                            prompt: question.prompt,
                            points: question.points,
                            order: question.order,
                            options,
                        };

                        Ok(question)
                    }
                    None => Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "Question not found.",
                    )),
                }
            }
            Err(sqlx::Error::RowNotFound) => {
                log::info!(
                    "question.get | service | get_quiz_question_by_id | failed | \"Question not found.\" | question_id={}",
                    id
                );

                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Question not found.",
                ))
            }
            Err(error) => {
                log::error!(
                    "question.get | service | get_quiz_question_by_id | failure | \"{:?}\" |",
                    error
                );

                Err(io::Error::other(error.to_string()))
            }
        }
    }

    // Options

    pub async fn create_option(&self, new_option: NewQuizQuestionOption) -> Result<Uuid, Error> {
        log::info!(
            "question_option.create.start | service | create_option | started | \"Creating question option.\" | question_id={}",
            new_option.question_id
        );

        match self.option_repo.create_option(new_option).await {
            Ok(id) => {
                log::info!(
                    "question_option.create.success | service | create_option | success | \"Created question option successfully.\" | option_id={}",
                    id
                );

                Ok(id)
            }
            Err(error) => {
                log::error!(
                    "question_option.create.failed | service | create_option | failed | \"Failed creating question option.\" | error=\"{}\"",
                    error
                );

                Err(Error::other(error.to_string()))
            }
        }
    }

    pub async fn get_options_by_question_id(
        &self,
        question_id: &Uuid,
    ) -> Result<Vec<QuizQuestionOption>, Error> {
        log::info!(
            "question_options.get.start | service | get_options_by_question_id | started | \"Getting options for question.\" | question_id={}",
            question_id
        );

        match self
            .option_repo
            .get_options_by_question_id(*question_id)
            .await
        {
            Ok(options) => {
                log::info!(
                    "question_options.get.success | service | get_options_by_question_id | success | \"Returned options successfully.\" | question_id={} | count={}",
                    question_id,
                    options.len()
                );

                Ok(options)
            }
            Err(error) => {
                log::error!(
                    "question_options.get.failed | service | get_options_by_question_id | failed | \"Failed retrieving options.\" | question_id={} | error=\"{}\"",
                    question_id,
                    error
                );

                Err(Error::other(error.to_string()))
            }
        }
    }

    pub async fn get_option_by_id(&self, id: Uuid) -> Result<QuizQuestionOption, Error> {
        log::info!(
            "question_option.get.start | service | get_option_by_id | started | \"Getting question option.\" | option_id={}",
            id
        );

        match self.option_repo.get_option_by_id(id).await {
            Ok(option) => {
                log::info!(
                    "question_option.get.success | service | get_option_by_id | success | \"Returned question option successfully.\" | option_id={}",
                    id
                );

                match option {
                    Some(option) => Ok(option),
                    None => Err(Error::new(
                        ErrorKind::NotFound,
                        "Question option not found.",
                    )),
                }
            }
            Err(sqlx::Error::RowNotFound) => {
                log::info!(
                    "question_option.get.failed | service | get_option_by_id | failed | \"Question option not found.\" | option_id={}",
                    id
                );

                Err(Error::new(
                    ErrorKind::NotFound,
                    "Question option not found.",
                ))
            }
            Err(error) => {
                log::error!(
                    "question_option.get.failed | service | get_option_by_id | failed | \"Failed retrieving question option.\" | option_id={} | error=\"{}\"",
                    id,
                    error
                );

                Err(Error::other(error.to_string()))
            }
        }
    }
}
