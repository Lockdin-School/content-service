use crate::infrastructure::db::database::{init_postgres, run_migrations};

#[derive(Clone)]
pub struct AppState {
    // access_modifier service_variable: Data<ServiceStruct>
}

pub fn app_state(// pg_pool: PgPool
) -> AppState {
    AppState {}
}

pub async fn init_state() -> AppState {
    log::info!("Initializing state...");
    let pg_pool = init_postgres().await;
    // let redis = init_redis().await.expect("Failed to initialize redis");

    run_migrations(&pg_pool).await;
    app_state(
        // pg_pool
    )
}
