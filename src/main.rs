mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod utils;

use config::Config;
use db::core::PgConnection;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use std::{env, sync::Arc};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let config = Config::default();

    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or(config.log_level),
    );

    let db_url = env::var("DATABASE_URL")?;
    let db = Arc::new(PgConnection::new(&db_url).await?);

    log::info!("Connected to database");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new((*db).clone()))
            .configure(routes::routes)
            .wrap(Logger::default())
    })
    .bind((config.addr, config.port))?
    .run()
    .await?;

    Ok(())
}
