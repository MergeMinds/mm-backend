mod config;
mod handlers;
mod models;
mod routes;

use actix_web::{middleware::Logger, App, HttpServer};
use config::Config;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let config = Config::default();

    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or(config.log_level),
    );

    HttpServer::new(|| {
        App::new().configure(routes::routes).wrap(Logger::default())
    })
    .bind((config.addr, config.port))?
    .run()
    .await?;

    Ok(())
}
