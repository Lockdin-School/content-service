use crate::configuration::events::event_handlers_init;
use crate::core::lessons::repository::LessonRepositoryImpl::PostgresLessonRepository;
use crate::core::lessons::service::LessonService::LessonService;
use crate::core::materials::repository::MaterialRepositoryImpl::PostgresMaterialRepository;
use crate::core::materials::service::MaterialService::MaterialService;
use crate::infrastructure::InternalEventBus::{EventBus, init_event_bus};
use crate::infrastructure::db::database::{init_postgres, run_migrations};
use actix_web::web::Data;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub material_service: Data<MaterialService>,
    pub lesson_service: Data<LessonService>,
}

pub fn app_state(pg_pool: PgPool, event_bus: Data<EventBus>) -> AppState {
    AppState {
        material_service: Data::new(MaterialService::new(Arc::new(
            PostgresMaterialRepository::new(pg_pool.clone()),
        ))),
        lesson_service: Data::new(LessonService::new(
            Arc::new(PostgresLessonRepository::new(pg_pool.clone())),
            event_bus.clone(),
        )),
    }
}

pub async fn init_state() -> AppState {
    log::info!(
        "application.state.init | configuration | init_state | started | \"Initializing state\" |"
    );
    let pg_pool = init_postgres().await;

    run_migrations(&pg_pool).await;

    let tx = init_event_bus();
    let event_bus = Data::new(tx.clone());

    let state = app_state(pg_pool, event_bus);

    // --- EVENT SUBSCRIBERS ---
    event_handlers_init(tx, Data::new(state.clone())).await;
    state
}
