mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod utils;

use config::Config;
use db::core::PgConnection;
use utils::core::Context;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use figment::{providers::Env, Figment};
use std::{env, sync::Arc};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let config: Config = Figment::from(Config::default())
        .merge(Env::prefixed("MM_"))
        .extract()?;

    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or(config.log_level.clone()),
    );

    let db_url = env::var("DATABASE_URL")?;
    let db = PgConnection::new(&db_url).await?;

    let ctx = Arc::new(Context {
        config: config.clone(),
        db,
    });

    log::info!("Connected to database");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new((*ctx).clone()))
            .configure(routes::routes)
            .wrap(Logger::default())
    })
    .bind((config.addr, config.port))?
    .run()
    .await?;

    Ok(())
}
