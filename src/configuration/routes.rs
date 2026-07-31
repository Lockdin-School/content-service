use crate::core::materials::handlers::get_all_materials;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        // ------------- configure routes ------------
        web::scope("/api/v1")
            .service(web::scope("/topics").service(get_all_materials))
            .service(web::scope("/materials"))
            .service(web::scope("/auth")),
    );
}
