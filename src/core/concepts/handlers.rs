use crate::configuration::state::AppState;
use crate::core::concepts::models::Concept::{ConceptNew, ConceptUpdate};
use actix_web::{
    HttpResponse, delete, get, post, put,
    web::{Data, Json, Path},
};
use uuid::Uuid;

// POST /concepts
#[post("")]
pub async fn create_concept(
    state: Data<AppState>,
    payload: Json<ConceptNew>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "concept.create.request | handler | create_concept | started | \"Creating concept.\" |"
    );
    match state
        .concept_service
        .create_concept(payload.into_inner())
        .await
    {
        Ok(concept) => {
            log::info!(
                "concept.create.success | handler | create_concept | success | \"Concept created successfully.\" |"
            );

            Ok(HttpResponse::Ok().json(concept))
        }
        Err(error) => {
            log::error!(
                "concept.create.failure | handler | create_concept | failure | \"{:?}\" |",
                error
            );
            Ok(HttpResponse::from_error(error))
        }
    }
}

// GET /concepts/{id}
#[get("/{id}")]
pub async fn get_concept_by_id(
    state: Data<AppState>,
    id: Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "concept.get.request | handler | get_concept_by_id | started | \"Getting concept by id.\" |"
    );
    match state
        .concept_service
        .get_concept_by_id(&id.into_inner())
        .await
    {
        Ok(concept) => {
            log::info!(
                "concept.get.success | handler | get_concept_by_id | success | \"Concept found.\" |"
            );

            Ok(HttpResponse::Ok().json(concept))
        }
        Err(error) => {
            log::error!(
                "concept.get.failure | handler | get_concept_by_id | failure | \"{:?}\" |",
                error
            );
            Ok(HttpResponse::from_error(error))
        }
    }
}

// GET /topics/{topic_id}/concepts
#[get("/{topic_id}/concepts")]
pub async fn get_concepts_by_topic_id(
    state: Data<AppState>,
    topic_id: Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "concepts.get.request | handler | get_concepts_by_topic_id | started | \"Getting concepts by topic id.\" |"
    );
    match state
        .concept_service
        .get_concepts_by_topic_id(&topic_id.into_inner())
        .await
    {
        Ok(concepts) => {
            log::info!(
                "concepts.get.success | handler | get_concepts_by_topic_id | success | \"Concepts found.\" |"
            );

            Ok(HttpResponse::Ok().json(concepts))
        }
        Err(error) => {
            log::error!(
                "concepts.get.failure | handler | get_concepts_by_topic_id | failure | \"{:?}\" |",
                error
            );
            Ok(HttpResponse::from_error(error))
        }
    }
}

// GET /concepts
#[get("")]
pub async fn get_all_concepts(state: Data<AppState>) -> actix_web::Result<HttpResponse> {
    log::info!(
        "concepts.get.request | handler | get_all_concepts | started | \"Getting all concepts.\" |"
    );
    match state.concept_service.get_all_concepts().await {
        Ok(concepts) => {
            log::info!(
                "concepts.get.success | handler | get_all_concepts | success | \"Concepts found.\" |"
            );

            Ok(HttpResponse::Ok().json(concepts))
        }
        Err(error) => {
            log::error!(
                "concepts.get.failure | handler | get_all_concepts | failure | \"{:?}\" |",
                error
            );
            Ok(HttpResponse::from_error(error))
        }
    }
}

// PUT /concepts/{id}
#[put("/{id}")]
pub async fn update_concept(
    state: Data<AppState>,
    id: Path<Uuid>,
    payload: Json<ConceptUpdate>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "concept.update.request | handler | update_concept | started | \"Updating concept.\" |"
    );
    match state
        .concept_service
        .update_concept(&id.into_inner(), payload.into_inner())
        .await
    {
        Ok(concept) => {
            log::info!(
                "concept.update.success | handler | update_concept | success | \"Concept updated successfully.\" |"
            );

            Ok(HttpResponse::Ok().json(concept))
        }
        Err(error) => {
            log::error!(
                "concept.update.failure | handler | update_concept | failure | \"{:?}\" |",
                error
            );
            Ok(HttpResponse::from_error(error))
        }
    }
}

// DELETE /concepts/{id}
#[delete("/{id}")]
pub async fn delete_concept(
    state: Data<AppState>,
    id: Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "concept.delete.request | handler | delete_concept | started | \"Deleting concept.\" |"
    );
    match state.concept_service.delete_concept(&id.into_inner()).await {
        Ok(()) => {
            log::info!(
                "concept.delete.success | handler | delete_concept | success | \"Concept deleted successfully.\" |"
            );
            Ok(HttpResponse::NoContent().finish())
        }
        Err(error) => {
            log::error!(
                "concept.delete.failure | handler | delete_concept | failure | \"{:?}\" |",
                error
            );
            Ok(HttpResponse::from_error(error))
        }
    }
}
