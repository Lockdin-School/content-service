use crate::configuration::server::run;
use crate::configuration::state::init_state;
use env_logger::Env;
use std::io::Write;

pub mod configuration;
pub mod core;
pub mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            writeln!(
                buf,
                "| {:<24} | {:<5} | {:<55} | {} |",
                buf.timestamp_millis(),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();

    // jsonwebtoken v10 requires a process-wide crypto provider to be installed
    // before any decode/verify call. This must happen once at startup.
    // let _ = jsonwebtoken::crypto::aws_lc::DEFAULT_PROVIDER.install_default(); // uncomment this
    //
    let state = init_state().await;

    run(state).await
}
