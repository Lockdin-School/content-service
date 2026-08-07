use crate::configuration::state::AppState;
use crate::core::materials::dto::MaterialResponseDTO::MaterialResponseDTO;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use uuid::Uuid;

#[get("/{topic_id}/materials")]
pub async fn get_all_materials(
    state: Data<AppState>,
    topic_id: Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "materials.get.request.received | handler | get_materials_by_topic | started | \"Received request to get topics\" |"
    );

    let topic_id = topic_id.into_inner();

    match state
        .material_service
        .get_materials_by_topic(&topic_id)
        .await
    {
        Ok(materials) => {
            log::info!(
                "materials.get.request.success | handler | get_materials_by_topic | success | \"Request to get materials processed successfully\" |"
            );
            let materials: Vec<MaterialResponseDTO> = materials
                .into_iter()
                .map(MaterialResponseDTO::from)
                .collect();
            Ok(HttpResponse::Ok().json(materials))
        }
        Err(error) => {
            log::error!(
                "materials.get.request.failed | handler | create_material | failed | \" Error: {error}\" |"
            );
            Ok(HttpResponse::from_error(error))
        }
    }
}

#[get("/{id}")]
pub async fn get_material_by_id(
    state: Data<AppState>,
    id: Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "material.get.request.received | handler | get_material_by_id | started | \"Received request to get material by id.\" |"
    );
    let id = id.into_inner();
    match state.material_service.get_material_by_id(&id).await {
        Ok(materials) => {
            log::info!(
                "material.get.request.success | handler | get_material_by_id | success | \"Received request to get material by id.\" |"
            );
            let material: MaterialResponseDTO = MaterialResponseDTO::from(materials);
            Ok(HttpResponse::Ok().json(material))
        }
        Err(error) => {
            log::error!(
                "material.get.request.failed | handler | get_material_by_id | failed | \" Error: {error}\" |"
            );
            Ok(HttpResponse::from_error(error))
        }
    }
}
