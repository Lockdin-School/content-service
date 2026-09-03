use crate::configuration::state::AppState;
use crate::core::lessons::dto::CreateLessonRequest::IncomingCreateLessonRequest;
use crate::core::lessons::dto::ReadLessonResponse::ReadLessonResponse;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, get, post};
use uuid::Uuid;

#[post("")]
pub async fn create_lesson(
    state: Data<AppState>,
    payload: Json<IncomingCreateLessonRequest>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "lesson.create.request | handler | create_lesson | started | \"Creating lesson.\" |"
    );
    match state
        .lesson_service
        .create_lesson(&payload.into_inner())
        .await
    {
        Ok(lesson) => {
            log::info!(
                "lesson.create.success | handler | create_lesson | success | \"Lesson created successfully.\" |"
            );
            let lesson = ReadLessonResponse::from(lesson);
            Ok(HttpResponse::Ok().json(lesson))
        }
        Err(error) => {
            log::error!(
                "lesson.create.failure | handler | create_lesson | failure | \"{:?}\" |",
                error
            );
            Ok(HttpResponse::from_error(error))
        }
    }
}

#[get("/{id}")]
pub async fn get_lesson_by_id(
    state: Data<AppState>,
    id: Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "lesson.get.request | handler | get_lesson_by_id | started | \"Getting lesson by id.\" |"
    );
    match state.lesson_service.get_lesson_by_id(id.into_inner()).await {
        Ok(lesson) => {
            log::info!(
                "lesson.get.success | handler | get_lesson_by_id | success | \"Lesson found.\" |"
            );
            let lesson = ReadLessonResponse::from(lesson);
            Ok(HttpResponse::Ok().json(lesson))
        }
        Err(error) => {
            log::error!(
                "lesson.get.failure | handler | get_lesson_by_id | failure | \"{:?}\" |",
                error
            );
            Ok(HttpResponse::from_error(error))
        }
    }
}
