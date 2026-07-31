use crate::configuration::state::AppState;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use uuid::Uuid;
use crate::core::materials::dto::MaterialResponseDTO::MaterialResponseDTO;

#[get("/{topic_id}/materials")]
pub async fn get_all_materials(
    state: Data<AppState>,
    topic_id: String,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "materials.get.request.received | handler | get_materials_by_topic | started | \"Received request to get topics\" |"
    );

    match Uuid::parse_str(&topic_id) {
        Ok(topic_id) => {
            match state
                .material_service
                .get_materials_by_topic(&topic_id)
                .await
            {
                Ok(materials) => {
                    log::info!(
                        "materials.get.request.success | handler | get_materials_by_topic | success | \"Request to get materials processed successfully\" |"
                    );
                    let materials: Vec<MaterialResponseDTO> = materials.into_iter().map(MaterialResponseDTO::from).collect();
                    Ok(HttpResponse::Ok().json(materials))
                }
                Err(error) => {
                    Ok(HttpResponse::from_error(error))
                }
            }
        }
        Err(e) => {
            log::error!(
                "materials.get.request.failed | handler | get_materials_by_topic | failed | \"Invalid topic id\" | \"{e}\" |"
            );
            Ok(HttpResponse::BadRequest().json("Invalid topic id"))
        }
    }
}
