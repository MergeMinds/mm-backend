use crate::auth;
use crate::course;
use crate::discipline;
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
        discipline::handlers::get_all,
        discipline::handlers::get_by_id,
        discipline::handlers::create_by_id,
        discipline::handlers::update_by_id,
        discipline::handlers::delete_by_id,
        course::handlers::get_all,
        course::handlers::get_by_id,
        course::handlers::create,
        course::handlers::update_by_id,
        course::handlers::delete_by_id,
    ),
    components(schemas(
            models::User,
            models::SignUpCredentials,
            models::SignInCredentials,
            models::Discipline,
            models::DisciplineIn,
            models::Course,
            models::CourseIn,
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
    .service(auth::handlers::logout)
    .service(discipline::handlers::get_all)
    .service(discipline::handlers::get_by_id)
    .service(discipline::handlers::create_by_id)
    .service(discipline::handlers::update_by_id)
    .service(discipline::handlers::delete_by_id)
    .service(course::handlers::get_all)
    .service(course::handlers::get_by_id)
    .service(course::handlers::create)
    .service(course::handlers::update_by_id)
    .service(course::handlers::delete_by_id);
}
