use std::sync::Arc;
use actix_web::web::Data;
use sqlx::PgPool;
use crate::core::materials::repository::MaterialRepositoryImpl::PostgresMaterialRepository;
use crate::core::materials::service::MaterialService::MaterialService;
use crate::infrastructure::db::database::{init_postgres, run_migrations};

#[derive(Clone)]
pub struct AppState {
    pub material_service: Data<MaterialService>
}

pub fn app_state(
    pg_pool: PgPool
) -> AppState {

    AppState {
        material_service: Data::new(
            MaterialService::new(
                Arc::new(
                    PostgresMaterialRepository::new(pg_pool)
                )
            )
        )
    }
}

pub async fn init_state() -> AppState {
    log::info!("Initializing state...");
    let pg_pool = init_postgres().await;

    run_migrations(&pg_pool).await;
    app_state(
        pg_pool
    )
}
