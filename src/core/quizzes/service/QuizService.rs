use crate::core::quizzes::dto::AggregateQuizAttempt::AggregateQuizAttempt;
use crate::core::quizzes::dto::OptionDTO::OptionDTO;
use crate::core::quizzes::dto::QuestionDTO::AggregateQuestion;
use crate::core::quizzes::dto::QuizDTO::AggregateQuiz;
use crate::core::quizzes::dto::bulk_dto::BulkQuizUpload;
use crate::core::quizzes::models::Quiz::NewQuiz;
use crate::core::quizzes::models::QuizAttempt::{QuizAttemptNew, QuizAttemptStatus};
use crate::core::quizzes::models::QuizQuestion::{
    NewQuizQuestion, NewQuizQuestionOption, QuizQuestionOption,
};
use crate::core::quizzes::models::QuizQuestionResponse::{
    EvaluatedQuizQuestionResponseNew, QuizQuestionResponseNew,
};
use crate::core::quizzes::models::outbound::Observation::{ItemObservation, Observation};
use crate::core::quizzes::outbound::publish_event::EventPublisher;
use crate::core::quizzes::quizzes_events::QuizCreatedPayload;
use crate::core::quizzes::repositories::interfaces::attempt::QuizAttemptRepository::QuizAttemptRepository;
use crate::core::quizzes::repositories::interfaces::option::QuestionOptionRepository::QuestionOptionRepository;
use crate::core::quizzes::repositories::interfaces::question::QuizQuestionRepository::QuizQuestionRepository;
use crate::core::quizzes::repositories::interfaces::quiz::QuizRepository::QuizRepository;
use crate::core::quizzes::repositories::interfaces::response::QuizQuestionResponseRepository::QuizQuestionResponseRepository;
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
    attempt_repo: Arc<dyn QuizAttemptRepository + Send + Sync>,
    response_repo: Arc<dyn QuizQuestionResponseRepository + Send + Sync>,
    event_bus: Data<EventBus>,
}

impl QuizService {
    pub fn new(
        repo: Arc<dyn QuizRepository + Send + Sync>,
        questions_repo: Arc<dyn QuizQuestionRepository + Send + Sync>,
        option_repo: Arc<dyn QuestionOptionRepository + Send + Sync>,
        attempt_repo: Arc<dyn QuizAttemptRepository + Send + Sync>,
        response_repo: Arc<dyn QuizQuestionResponseRepository + Send + Sync>,
        event_bus: Data<EventBus>,
    ) -> Self {
        QuizService {
            repo,
            questions_repo,
            option_repo,
            attempt_repo,
            response_repo,
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
                    estimated_duration_seconds: Some(quiz.estimated_duration_seconds),
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

    pub async fn upload_quiz_bulk(
        &self,
        payload: BulkQuizUpload,
    ) -> Result<AggregateQuiz, io::Error> {
        log::info!(
            "quiz.bulk_upload.start | service | upload_quiz_bulk | started | \"Uploading full quiz.\" | title=\"{}\" | questions_count={}",
            payload.quiz.title,
            payload.questions.len()
        );

        if payload.questions.is_empty() {
            log::warn!(
                "quiz.bulk_upload.failed | service | upload_quiz_bulk | failed | \"No questions provided.\" |"
            );

            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Quiz must contain at least one question.",
            ));
        }

        let quiz_id = self.create_quiz(payload.quiz).await?;

        for question in payload.questions {
            if question.options.is_empty() {
                log::warn!(
                    "quiz.bulk_upload.failed | service | upload_quiz_bulk | failed | \"Question has no options.\" | quiz_id={} | order={}",
                    quiz_id,
                    question.order
                );

                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Each question must contain at least one option.",
                ));
            }

            let question_id = self
                .create_question(NewQuizQuestion {
                    quiz_id,
                    concept_id: question.material_id,
                    question_type: question.question_type,
                    prompt: question.prompt,
                    points: question.points,
                    order: question.order,
                })
                .await?;

            for option in question.options {
                self.create_option(NewQuizQuestionOption {
                    question_id,
                    text: option.text,
                    is_correct: option.is_correct,
                    order: option.order,
                })
                .await?;
            }
        }

        let aggregate_quiz = self.get_quiz_by_id(quiz_id).await?;

        log::info!(
            "quiz.bulk_upload.success | service | upload_quiz_bulk | success | \"Uploaded full quiz successfully.\" | quiz_id={}",
            quiz_id
        );

        Ok(aggregate_quiz)
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
                    title: quiz_db.title,
                    description: quiz_db.description,
                    difficulty: quiz_db.difficulty,
                    passing_score: quiz_db.passing_score,
                    estimated_duration_seconds: quiz_db.estimated_duration_seconds,
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
                            concept_id: question.concept_id,
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

    /// Quiz Attempts
    pub async fn create_quiz_attempt(
        &self,
        attempt: QuizAttemptNew,
    ) -> Result<AggregateQuizAttempt, io::Error> {
        log::info!(
            "quiz_attempt.create.start | service | create_quiz_attempt | started | \"Creating quiz attempt.\" | quiz_id={} | student_id={}",
            attempt.quiz_id,
            attempt.student_id
        );

        let quiz_id = attempt.quiz_id;
        let student_id = attempt.student_id;

        match self.attempt_repo.save(attempt).await {
            Ok(_) => {
                self.get_quiz_attempt_by_student_id(quiz_id, student_id)
                    .await
            }
            Err(error) => {
                log::error!(
                    "quiz_attempt.create.failed | service | create_quiz_attempt | failed | \"Failed creating quiz attempt.\" | quiz_id={} | student_id={} | error=\"{}\"",
                    quiz_id,
                    student_id,
                    error
                );

                Err(io::Error::other(error.to_string()))
            }
        }
    }

    pub async fn get_quiz_attempt_by_student_id(
        &self,
        quiz_id: Uuid,
        student_id: Uuid,
    ) -> Result<AggregateQuizAttempt, io::Error> {
        log::info!(
            "quiz_attempt.get.start | service | get_quiz_attempt_by_student_id | started | \"Getting quiz attempt.\" | quiz_id={} | student_id={}",
            quiz_id,
            student_id
        );

        match self
            .attempt_repo
            .get_quiz_attempt_by_student_id(quiz_id, student_id)
            .await
        {
            Ok(attempt) => {
                let responses = self
                    .response_repo
                    .get_responses_by_attempt_id(attempt.id)
                    .await
                    .map_err(|error| {
                        log::error!(
                            "quiz_attempt.get_responses.failed | service | get_quiz_attempt_by_student_id | failed | \"Failed getting quiz attempt responses.\" | attempt_id={} | error=\"{}\"",
                            attempt.id,
                            error
                        );

                        io::Error::other(error.to_string())
                    })?;

                Ok(AggregateQuizAttempt::from_attempt_and_responses(
                    attempt, responses,
                ))
            }
            Err(sqlx::Error::RowNotFound) => {
                log::info!(
                    "quiz_attempt.get.not_found | service | get_quiz_attempt_by_student_id | failed | \"Quiz attempt not found.\" | quiz_id={} | student_id={}",
                    quiz_id,
                    student_id
                );

                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Quiz attempt not found.",
                ))
            }
            Err(error) => {
                log::error!(
                    "quiz_attempt.get.failed | service | get_quiz_attempt_by_student_id | failed | \"Failed getting quiz attempt.\" | quiz_id={} | student_id={} | error=\"{}\"",
                    quiz_id,
                    student_id,
                    error
                );

                Err(io::Error::other(error.to_string()))
            }
        }
    }

    pub async fn update_quiz_attempt(
        &self,
        id: Uuid,
        mut attempt: QuizAttemptNew,
    ) -> Result<AggregateQuizAttempt, io::Error> {
        log::info!(
            "quiz_attempt.update.start | service | update_quiz_attempt | started | \"Updating quiz attempt.\" | attempt_id={}",
            id
        );

        if attempt.status == QuizAttemptStatus::Completed {
            let (score, percentage) = self.calculate_attempt_score(id, attempt.quiz_id).await?;

            attempt.score = Some(score);
            attempt.percentage = Some(percentage);

            if attempt.ended_at.is_none() {
                attempt.ended_at = Some(chrono::Utc::now());
            }
        }

        match self.attempt_repo.update_attempt(id, attempt).await {
            Ok(Some(updated_attempt)) => {
                let responses = self
                    .response_repo
                    .get_responses_by_attempt_id(updated_attempt.id)
                    .await
                    .map_err(|error| {
                        log::error!(
                            "quiz_attempt.update_responses.failed | service | update_quiz_attempt | failed | \"Failed getting quiz attempt responses.\" | attempt_id={} | error=\"{}\"",
                            updated_attempt.id,
                            error
                        );

                        io::Error::other(error.to_string())
                    })?;

                // potential violation
                let quiz = self
                    .repo
                    .get_quiz_by_id(updated_attempt.quiz_id)
                    .await
                    .unwrap();

                if updated_attempt.status == QuizAttemptStatus::Completed {
                    let items = responses
                        .iter()
                        .map(|response| {
                            ItemObservation {
                                question_id: response.question_id,
                                concept_id: response.concept_id,
                                correct: response.is_correct,
                                answered_at: response.answered_at,
                            }
                        })
                        .collect::<Vec<ItemObservation>>();

                    let observation = Observation {
                        event_type: "quiz_attempt_graded".to_string(),
                        version: 1,
                        student_id: updated_attempt.clone().student_id,
                        occurred_at: updated_attempt.ended_at.unwrap_or_default(),
                        source_service: "content-service".to_string(),
                        source_event_id: updated_attempt.id,
                        data: serde_json::json!({
                            "quiz_id": updated_attempt.quiz_id,
                            "attempt_id": updated_attempt.id,
                            "subject_id": quiz.subject_id,
                            "topic_id": quiz.topic_id,
                            "score": updated_attempt.score.unwrap_or(0),
                            "passing_score": quiz.passing_score,
                            "max_score": quiz.max_score,
                            "percentage": updated_attempt.percentage,
                            "items": items,
                        }),
                    };

                    log::info!(
                        "quiz_service.update_quiz_attempt | service | publish_event | initiating | \"Publishing quiz attempted grade event.\" | attempt_id={}",
                        updated_attempt.id
                    );

                    match EventPublisher::publish_event(&observation).await {
                        Ok(_) => {
                            log::info!(
                                "event.publish.success | service | publish_event | success | \"Quiz attempted grade event published successfully.\" | attempt_id={}",
                                updated_attempt.id
                            );
                        }
                        Err(e) => {
                            log::error!(
                                "event.publish.failed | service | publish_event | failed | \"Failed publishing quiz attempted grade event.\" | attempt_id={} | error=\"{}\"",
                                updated_attempt.id,
                                e
                            );
                        }
                    }
                }

                Ok(AggregateQuizAttempt::from_attempt_and_responses(
                    updated_attempt,
                    responses,
                ))
            }
            Ok(None) => {
                log::info!(
                    "quiz_attempt.update.not_found | service | update_quiz_attempt | failed | \"Quiz attempt not found.\" | attempt_id={}",
                    id
                );

                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Quiz attempt not found.",
                ))
            }
            Err(error) => {
                log::error!(
                    "quiz_attempt.update.failed | service | update_quiz_attempt | failed | \"Failed updating quiz attempt.\" | attempt_id={} | error=\"{}\"",
                    id,
                    error
                );

                Err(io::Error::other(error.to_string()))
            }
        }
    }

    pub async fn save_quiz_question_response(
        &self,
        response: QuizQuestionResponseNew,
    ) -> Result<AggregateQuizAttempt, io::Error> {
        log::info!(
            "quiz_question_response.save.start | service | save_quiz_question_response | started | \"Saving quiz question response.\" | attempt_id={} | question_id={}",
            response.attempt_id,
            response.question_id
        );

        let attempt_id = response.attempt_id;

        let evaluated_response = EvaluatedQuizQuestionResponseNew {
            attempt_id: response.attempt_id,
            question_id: response.question_id,
            concept_id: response.concept_id,
            selected_option_id: response.selected_option_id,
            is_correct: self
                .is_selected_option_correct(response.selected_option_id)
                .await?,
            answered_at: response.answered_at,
        };

        match self.response_repo.save(evaluated_response).await {
            Ok(_) => self.get_aggregate_attempt_by_attempt_id(attempt_id).await,
            Err(error) => {
                log::error!(
                    "quiz_question_response.save.failed | service | save_quiz_question_response | failed | \"Failed saving quiz question response.\" | attempt_id={} | error=\"{}\"",
                    attempt_id,
                    error
                );

                Err(io::Error::other(error.to_string()))
            }
        }
    }

    pub async fn update_quiz_question_response(
        &self,
        id: Uuid,
        response: QuizQuestionResponseNew,
    ) -> Result<AggregateQuizAttempt, io::Error> {
        log::info!(
            "quiz_question_response.update.start | service | update_quiz_question_response | started | \"Updating quiz question response.\" | response_id={} | attempt_id={} | question_id={}",
            id,
            response.attempt_id,
            response.question_id
        );

        let attempt_id = response.attempt_id;

        let evaluated_response = EvaluatedQuizQuestionResponseNew {
            attempt_id: response.attempt_id,
            question_id: response.question_id,
            concept_id: response.concept_id,
            selected_option_id: response.selected_option_id,
            is_correct: self
                .is_selected_option_correct(response.selected_option_id)
                .await?,
            answered_at: response.answered_at,
        };

        match self
            .response_repo
            .update_response(id, evaluated_response)
            .await
        {
            Ok(Some(_)) => self.get_aggregate_attempt_by_attempt_id(attempt_id).await,
            Ok(None) => {
                log::info!(
                    "quiz_question_response.update.not_found | service | update_quiz_question_response | failed | \"Quiz question response not found.\" | response_id={}",
                    id
                );

                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Quiz question response not found.",
                ))
            }
            Err(error) => {
                log::error!(
                    "quiz_question_response.update.failed | service | update_quiz_question_response | failed | \"Failed updating quiz question response.\" | response_id={} | error=\"{}\"",
                    id,
                    error
                );

                Err(io::Error::other(error.to_string()))
            }
        }
    }

    async fn is_selected_option_correct(
        &self,
        selected_option_id: Option<Uuid>,
    ) -> Result<bool, io::Error> {
        let selected_option_id = match selected_option_id {
            Some(id) => id,
            None => return Ok(false),
        };

        match self.option_repo.get_option_by_id(selected_option_id).await {
            Ok(Some(option)) => Ok(option.is_correct),
            Ok(None) => {
                log::info!(
                    "quiz_question_response.evaluate.not_found | service | is_selected_option_correct | failed | \"Selected option not found.\" | option_id={}",
                    selected_option_id
                );

                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Selected option not found.",
                ))
            }
            Err(error) => {
                log::error!(
                    "quiz_question_response.evaluate.failed | service | is_selected_option_correct | failed | \"Failed evaluating selected option.\" | option_id={} | error=\"{}\"",
                    selected_option_id,
                    error
                );

                Err(io::Error::other(error.to_string()))
            }
        }
    }

    async fn calculate_attempt_score(
        &self,
        attempt_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<(i32, f64), io::Error> {
        let responses = self
            .response_repo
            .get_responses_by_attempt_id(attempt_id)
            .await
            .map_err(|error| {
                log::error!(
                        "quiz_attempt.score.responses.failed | service | calculate_attempt_score | failed | \"Failed getting attempt responses.\" | attempt_id={} | error=\"{}\"",
                        attempt_id,
                        error
                    );

                io::Error::other(error.to_string())
            })?;

        let questions = self
            .questions_repo
            .get_quiz_questions_by_quiz_id(quiz_id)
            .await
            .map_err(|error| {
                log::error!(
                        "quiz_attempt.score.questions.failed | service | calculate_attempt_score | failed | \"Failed getting quiz questions.\" | quiz_id={} | error=\"{}\"",
                        quiz_id,
                        error
                    );

                io::Error::other(error.to_string())
            })?;

        let total_points: i32 = questions.iter().map(|question| question.points).sum();

        if total_points == 0 {
            return Ok((0, 0.0));
        }

        let mut score = 0;

        for response in responses {
            if !response.is_correct {
                continue;
            }

            let question = questions
                .iter()
                .find(|question| question.id == response.question_id);

            if let Some(question) = question {
                score += question.points;
            }
        }

        let percentage = (score as f64 / total_points as f64) * 100.0;

        log::info!(
            "quiz_attempt.score.success | service | calculate_attempt_score | success | \"Calculated attempt score.\" | attempt_id={} | score={} | total_points={} | percentage={}",
            attempt_id,
            score,
            total_points,
            percentage
        );

        Ok((score, percentage))
    }

    async fn get_aggregate_attempt_by_attempt_id(
        &self,
        attempt_id: Uuid,
    ) -> Result<AggregateQuizAttempt, io::Error> {
        let attempt = self
            .attempt_repo
            .get_quiz_attempt_by_id(attempt_id)
            .await
            .map_err(|error| {
                log::error!(
                    "quiz_attempt.aggregate.get_attempt.failed | service | get_aggregate_attempt_by_attempt_id | failed | \"Failed getting quiz attempt.\" | attempt_id={} | error=\"{}\"",
                    attempt_id,
                    error
                );

                io::Error::other(error.to_string())
            })?;

        let responses = self
            .response_repo
            .get_responses_by_attempt_id(attempt_id)
            .await
            .map_err(|error| {
                log::error!(
                    "quiz_attempt.aggregate.get_responses.failed | service | get_aggregate_attempt_by_attempt_id | failed | \"Failed getting quiz attempt responses.\" | attempt_id={} | error=\"{}\"",
                    attempt_id,
                    error
                );

                io::Error::other(error.to_string())
            })?;

        Ok(AggregateQuizAttempt::from_attempt_and_responses(
            attempt, responses,
        ))
    }
}
