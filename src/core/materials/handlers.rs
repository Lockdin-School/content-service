use crate::configuration::state::AppState;
use crate::core::materials::dto::CreateMaterialRequestDTO::CreateMaterialRequest;
use crate::core::materials::dto::MaterialResponseDTO::MaterialResponseDTO;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, get, post};
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

#[post("")]
pub async fn create_material(
    state: Data<AppState>,
    material: Json<CreateMaterialRequest>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "material.post.request.received | handler | create_material | started | \"Received request to create material.\" |"
    );
    let material = material.into_inner();
    match state.material_service.create_material(material).await {
        Ok(materials) => {
            log::info!(
                "material.post.request.success | handler | create_material | success | \"Received request to create material.\" |"
            );
            let material: MaterialResponseDTO = MaterialResponseDTO::from(materials);
            Ok(HttpResponse::Ok().json(material))
        }
        Err(error) => {
            log::error!(
                "material.post.request.failed | handler | create_material | failed | \" Error: {error}\" |"
            );
            Ok(HttpResponse::from_error(error))
        }
    }
}
