use crate::configuration::state::AppState;
use crate::core::quizzes::models::Quiz::NewQuiz;
use crate::core::quizzes::models::QuizQuestion::{NewQuizQuestion, NewQuizQuestionOption};
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, get, post};
use uuid::Uuid;

#[post("")]
pub async fn create_quiz(
    state: Data<AppState>,
    quiz: Result<Json<NewQuiz>, actix_web::Error>,
) -> actix_web::Result<HttpResponse> {
    let quiz = match quiz {
        Ok(quiz) => quiz,
        Err(error) => {
            log::error!(
                "quiz.create | handler | create_quiz | failure | JSON deserialization failed: {:?}",
                error
            );

            return Err(error);
        }
    };

    log::info!("quiz.create | handler | create_quiz | start | \"Creating quiz...\" |");

    match state.quiz_service.create_quiz(quiz.into_inner()).await {
        Ok(id) => {
            log::info!(
                "quiz.create | handler | create_quiz | success | \"Quiz created with id {}\" |",
                id
            );

            Ok(HttpResponse::Ok().json(id))
        }

        Err(error) => {
            log::error!(
                "quiz.create | handler | create_quiz | failure | Failed to create quiz: {:?} |",
                error
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}

#[get("/{id}")]
pub async fn get_quiz_by_id(
    state: Data<AppState>,
    id: Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    match state.quiz_service.get_quiz_by_id(id.into_inner()).await {
        Ok(quiz) => {
            log::info!(
                "quiz.get | handler | get_quiz_by_id | success | \"Quiz retrieved successfully\" |"
            );
            Ok(HttpResponse::Ok().json(quiz))
        }
        Err(error) => {
            log::error!(
                "quiz.get | handler | get_quiz_by_id | failure | Failed to get quiz by id: {:?} |",
                error
            );
            Ok(HttpResponse::from_error(error))
        }
    }
}

// ------------------------
// POST /questions
#[post("")]
pub async fn create_question(
    state: Data<AppState>,
    question: Json<NewQuizQuestion>,
) -> actix_web::Result<HttpResponse> {
    log::info!("question.create | handler | create_question | started | \"Creating question.\" |");

    match state
        .quiz_service
        .create_question(question.into_inner())
        .await
    {
        Ok(id) => {
            log::info!(
                "question.create | handler | create_question | success | \"Question created with id {}\" |",
                id
            );

            Ok(HttpResponse::Ok().json(id))
        }
        Err(error) => {
            log::error!(
                "question.create | handler | create_question | failure | \"{:?}\" |",
                error
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}

// POST /options
#[post("")]
pub async fn create_option(
    state: Data<AppState>,
    payload: Json<NewQuizQuestionOption>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "question_option.create.request | handler | create_option | started | \"Creating question option.\" |"
    );

    match state
        .quiz_service
        .create_option(payload.into_inner())
        .await
    {
        Ok(id) => {
            log::info!(
                "question_option.create.success | handler | create_option | success | \"Question option created successfully.\" |"
            );

            Ok(HttpResponse::Ok().json(id))
        }
        Err(error) => {
            log::error!(
                "question_option.create.failure | handler | create_option | failure | \"{:?}\" |",
                error
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}

// GET /questions/{question_id}/options
#[get("/{question_id}/options")]
pub async fn get_options_by_question_id(
    state: Data<AppState>,
    question_id: Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "question_options.get.request | handler | get_options_by_question_id | started | \"Getting options by question id.\" |"
    );

    match state
        .quiz_service
        .get_options_by_question_id(&question_id.into_inner())
        .await
    {
        Ok(options) => {
            log::info!(
                "question_options.get.success | handler | get_options_by_question_id | success | \"Options found.\" |"
            );

            Ok(HttpResponse::Ok().json(options))
        }
        Err(error) => {
            log::error!(
                "question_options.get.failure | handler | get_options_by_question_id | failure | \"{:?}\" |",
                error
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}

// GET /options/{id}
#[get("/{id}")]
pub async fn get_option_by_id(
    state: Data<AppState>,
    id: Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "question_option.get.request | handler | get_option_by_id | started | \"Getting question option by id.\" |"
    );

    match state
        .quiz_service
        .get_option_by_id(id.into_inner())
        .await
    {
        Ok(option) => {
            log::info!(
                "question_option.get.success | handler | get_option_by_id | success | \"Question option found.\" |"
            );

            Ok(HttpResponse::Ok().json(option))
        }
        Err(error) => {
            log::error!(
                "question_option.get.failure | handler | get_option_by_id | failure | \"{:?}\" |",
                error
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}