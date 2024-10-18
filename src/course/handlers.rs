use crate::{context::Context, models};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful getting of courses"),
    )
)]
#[get("/courses")]
async fn get_all(ctx: Data<Context>) -> HttpResponse {
    let Ok(courses) = ctx.db.get_courses().await else {
        return HttpResponse::InternalServerError().finish();
    };

    HttpResponse::Ok().json(courses)
}

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful getting of course by id"),
        (status = StatusCode::NOT_FOUND, description = "Course not found"),
    ),
    params(
        ("id" = Uuid, Path, description = "ID"),
    )
)]
#[get("/courses/{id}")]
async fn get_by_id(ctx: Data<Context>, id: Path<Uuid>) -> HttpResponse {
    match ctx.db.get_course_by_id(*id).await {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

#[utoipa::path(
    request_body = CourseIn,
    responses(
        (status = StatusCode::OK, description = "Successful creating of course"),
    ),
    params(
        ("id" = Uuid, Path, description = "ID"),
    )
)]
#[post("/courses")]
async fn create(
    ctx: Data<Context>,
    Json(course): Json<models::CourseIn>,
) -> HttpResponse {
    match ctx.db.add_course(course).await {
        Ok(course) => HttpResponse::Created().json(course),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    request_body = CourseIn,
    responses(
        (status = StatusCode::OK, description = "Successful updating of course by id"),
        (status = StatusCode::NOT_FOUND, description = "Course not found"),
    ),
    params(
        ("id" = Uuid, Path, description = "ID"),
    )
)]
#[put("/courses/{id}")]
async fn update_by_id(
    ctx: Data<Context>,
    id: Path<Uuid>,
    Json(course): Json<models::CourseIn>,
) -> HttpResponse {
    match ctx.db.update_course_by_id(*id, course).await {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful deleting of course by id"),
        (status = StatusCode::NOT_FOUND, description = "Course not found"),
    ),
    params(
        ("id" = Uuid, Path, description = "ID"),
    )
)]
#[delete("/courses/{id}")]
async fn delete_by_id(ctx: Data<Context>, id: Path<Uuid>) -> HttpResponse {
    match ctx.db.delete_course_by_id(*id).await {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
