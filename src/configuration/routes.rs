use crate::core::lessons::handlers::create_lesson;
use crate::core::materials::handlers::{get_all_materials, get_material_by_id};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        // ------------- configure routes ------------
        web::scope("/api/v1")
            .service(web::scope("/topics").service(get_all_materials))
            .service(web::scope("/materials").service(get_material_by_id))
            .service(web::scope("/lessons").service(create_lesson))
            .service(web::scope("/auth")),
    );
}
