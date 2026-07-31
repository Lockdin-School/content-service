use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

// pub async fn init_redis() -> RedisResult<redis::aio::MultiplexedConnection> {
//     let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
//     log::info!("Connecting to Redis at {redis_url}");
//     match redis::Client::open(redis_url) {
//         Ok(client) => {
//             match client.get_multiplexed_async_connection().await {
//                 Ok(conn) => Ok(conn),
//                 Err(e) => {
//                     log::error!("Failed to connect to Redis: {e}");
//                     Err(e)
//                 }
//             }
//         },
//         Err(e) => {
//             log::info!("Failed to connect to Redis: {e}");
//             Err(e)
//         }
//     }
// }

pub async fn init_postgres() -> PgPool {
    let database_url = std::env::var("DATABASE_URL_INTERNAL").expect("DATABASE_URL must be set");
    log::info!(
        "database.postgres.connection.start | database | init_postgres | started | \"Connecting to Database\" |"
    );
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create PgPool")
}

pub async fn run_migrations(pool: &PgPool) {
    log::info!(
        "database.migrations.run.start | database | run_migrations | started | \"Running database migrations\" |"
    );
    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => {
            log::info!(
                "database.migrations.run.success | database | run_migrations | success | \"Database migrations ran successfully\" | path=\"./migrations\""
            );
        }
        Err(e) => {
            log::error!(
                "database.migrations.run.failed | database | run_migrations | failed | \"Failed to run database migrations\" | error=\"{e}\" path=\"./migrations\""
            );
        }
    }
}
