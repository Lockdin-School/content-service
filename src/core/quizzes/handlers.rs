use crate::configuration::state::AppState;
use crate::core::quizzes::models::Quiz::NewQuiz;
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
