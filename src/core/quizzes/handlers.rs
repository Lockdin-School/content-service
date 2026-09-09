use crate::configuration::state::AppState;
use crate::core::quizzes::dto::bulk_dto::BulkQuizUpload;
use crate::core::quizzes::models::Quiz::NewQuiz;
use crate::core::quizzes::models::QuizAttempt::QuizAttemptNew;
use crate::core::quizzes::models::QuizQuestion::{NewQuizQuestion, NewQuizQuestionOption};
use crate::core::quizzes::models::QuizQuestionResponse::QuizQuestionResponseNew;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, get, post, put};
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

    match state.quiz_service.create_option(payload.into_inner()).await {
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

    match state.quiz_service.get_option_by_id(id.into_inner()).await {
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

// POST /quiz-attempts
#[post("")]
pub async fn create_quiz_attempt(
    state: Data<AppState>,
    payload: Json<QuizAttemptNew>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "quiz_attempt.create.request | handler | create_quiz_attempt | started | \"Creating quiz attempt.\" |"
    );

    match state
        .quiz_service
        .create_quiz_attempt(payload.into_inner())
        .await
    {
        Ok(attempt) => {
            log::info!(
                "quiz_attempt.create.success | handler | create_quiz_attempt | success | \"Quiz attempt created successfully.\" | attempt_id={}",
                attempt.id
            );

            Ok(HttpResponse::Ok().json(attempt))
        }
        Err(error) => {
            log::error!(
                "quiz_attempt.create.failure | handler | create_quiz_attempt | failure | \"{:?}\" |",
                error
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}

#[post("/bulk")]
pub async fn upload_quiz_bulk(
    state: Data<AppState>,
    payload: Result<Json<BulkQuizUpload>, actix_web::Error>,
) -> actix_web::Result<HttpResponse> {
    let payload = match payload {
        Ok(payload) => payload,
        Err(error) => {
            log::error!(
                "quiz.bulk_upload | handler | upload_quiz_bulk | failure | JSON deserialization failed: {:?}",
                error
            );

            return Err(error);
        }
    };

    log::info!(
        "quiz.bulk_upload | handler | upload_quiz_bulk | started | \"Uploading full quiz.\" |"
    );

    match state
        .quiz_service
        .upload_quiz_bulk(payload.into_inner())
        .await
    {
        Ok(quiz) => {
            log::info!(
                "quiz.bulk_upload | handler | upload_quiz_bulk | success | \"Full quiz uploaded successfully.\" | quiz_id={}",
                quiz.id
            );

            Ok(HttpResponse::Ok().json(quiz))
        }
        Err(error) => {
            log::error!(
                "quiz.bulk_upload | handler | upload_quiz_bulk | failure | Failed to upload full quiz: {:?} |",
                error
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}

// GET /quiz-attempts/quiz/{quiz_id}/student/{student_id}
#[get("/quiz/{quiz_id}/student/{student_id}")]
pub async fn get_quiz_attempt_by_student_id(
    state: Data<AppState>,
    path: Path<(Uuid, Uuid)>,
) -> actix_web::Result<HttpResponse> {
    let (quiz_id, student_id) = path.into_inner();

    log::info!(
        "quiz_attempt.get.request | handler | get_quiz_attempt_by_student_id | started | \"Getting quiz attempt by quiz and student.\" | quiz_id={} | student_id={}",
        quiz_id,
        student_id
    );

    match state
        .quiz_service
        .get_quiz_attempt_by_student_id(quiz_id, student_id)
        .await
    {
        Ok(attempt) => {
            log::info!(
                "quiz_attempt.get.success | handler | get_quiz_attempt_by_student_id | success | \"Quiz attempt found.\" | attempt_id={}",
                attempt.id
            );

            Ok(HttpResponse::Ok().json(attempt))
        }
        Err(error) => {
            log::error!(
                "quiz_attempt.get.failure | handler | get_quiz_attempt_by_student_id | failure | \"{:?}\" |",
                error
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}

// PUT /quiz-attempts/{id}
#[put("/{id}")]
pub async fn update_quiz_attempt(
    state: Data<AppState>,
    id: Path<Uuid>,
    payload: Json<QuizAttemptNew>,
) -> actix_web::Result<HttpResponse> {
    let attempt_id = id.into_inner();

    log::info!(
        "quiz_attempt.update.request | handler | update_quiz_attempt | started | \"Updating quiz attempt.\" | attempt_id={}",
        attempt_id
    );

    match state
        .quiz_service
        .update_quiz_attempt(attempt_id, payload.into_inner())
        .await
    {
        Ok(attempt) => {
            log::info!(
                "quiz_attempt.update.success | handler | update_quiz_attempt | success | \"Quiz attempt updated successfully.\" | attempt_id={}",
                attempt.id
            );

            Ok(HttpResponse::Ok().json(attempt))
        }
        Err(error) => {
            log::error!(
                "quiz_attempt.update.failure | handler | update_quiz_attempt | failure | \"{:?}\" |",
                error
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}

// POST /question-responses
#[post("")]
pub async fn save_quiz_question_response(
    state: Data<AppState>,
    payload: Json<QuizQuestionResponseNew>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "quiz_question_response.save.request | handler | save_quiz_question_response | started | \"Saving quiz question response.\" |"
    );

    match state
        .quiz_service
        .save_quiz_question_response(payload.into_inner())
        .await
    {
        Ok(attempt) => {
            log::info!(
                "quiz_question_response.save.success | handler | save_quiz_question_response | success | \"Quiz question response saved successfully.\" | attempt_id={}",
                attempt.id
            );

            Ok(HttpResponse::Ok().json(attempt))
        }
        Err(error) => {
            log::error!(
                "quiz_question_response.save.failure | handler | save_quiz_question_response | failure | \"{:?}\" |",
                error
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}

// PUT /question-responses/{id}
#[put("/{id}")]
pub async fn update_quiz_question_response(
    state: Data<AppState>,
    id: Path<Uuid>,
    payload: Json<QuizQuestionResponseNew>,
) -> actix_web::Result<HttpResponse> {
    let response_id = id.into_inner();

    log::info!(
        "quiz_question_response.update.request | handler | update_quiz_question_response | started | \"Updating quiz question response.\" | response_id={}",
        response_id
    );

    match state
        .quiz_service
        .update_quiz_question_response(response_id, payload.into_inner())
        .await
    {
        Ok(attempt) => {
            log::info!(
                "quiz_question_response.update.success | handler | update_quiz_question_response | success | \"Quiz question response updated successfully.\" | attempt_id={}",
                attempt.id
            );

            Ok(HttpResponse::Ok().json(attempt))
        }
        Err(error) => {
            log::error!(
                "quiz_question_response.update.failure | handler | update_quiz_question_response | failure | \"{:?}\" |",
                error
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}
