use crate::handlers::auth;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::register)
        .service(auth::login)
        .service(auth::refresh);
}
