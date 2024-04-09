mod config;

use actix_web::{
    get, middleware::Logger, App, HttpResponse, HttpServer, Responder,
};

use config::Config;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let config = Config::default();

    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or(config.log_level),
    );

    HttpServer::new(|| App::new().service(hello).wrap(Logger::default()))
        .bind((config.addr, config.port))?
        .run()
        .await?;

    Ok(())
}
