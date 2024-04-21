use crate::auth;
use crate::models;

use actix_web::web;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::handlers::register,
        auth::handlers::login,
        auth::handlers::refresh,
        auth::handlers::logout,
    ),
    components(schemas(
            models::User,
            models::SignUpCredentials,
            models::SignInCredentials,
    )),
    tags(
        (name = "auth", description = "Authorization API")
    ),
)]
struct ApiDocs;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        utoipa_rapidoc::RapiDoc::with_openapi(
            "/api-docs/openapi.json",
            ApiDocs::openapi(),
        )
        .path("/docs"),
    )
    .service(auth::handlers::register)
    .service(auth::handlers::login)
    .service(auth::handlers::refresh)
    .service(auth::handlers::logout);
}
