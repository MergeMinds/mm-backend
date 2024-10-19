use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use uuid::Uuid;

use crate::{context::Context, models};

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful getting of disciplines"),
    )
)]
#[get("/disciplines")]
async fn get_all(ctx: Data<Context>) -> HttpResponse {
    match ctx.db.get_disciplines().await {
        Ok(disciplines) => HttpResponse::Ok().json(disciplines),
        Err(e) => {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful getting of discipline by id"),
        (status = StatusCode::NOT_FOUND, description = "Discipline not found"),
    ),
    params(
        ("id" = Uuid, Path, description = "ID"),
    )
)]
#[get("/disciplines/{id}")]
async fn get_by_id(ctx: Data<Context>, id: Path<Uuid>) -> HttpResponse {
    match ctx.db.get_discipline_by_id(*id).await {
        Ok(discipline) => HttpResponse::Ok().json(discipline),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
            _ => {
                log::error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

#[utoipa::path(
    request_body = DisciplineIn,
    responses(
        (status = StatusCode::OK, description = "Successful creating of discipline"),
    ),
    params(
        ("id" = Uuid, Path, description = "ID"),
    )
)]
#[post("/disciplines")]
async fn create(
    ctx: Data<Context>,
    Json(discipline): Json<models::DisciplineIn>,
) -> HttpResponse {
    match ctx.db.add_discipline(&discipline.name).await {
        Ok(discipline) => HttpResponse::Created().json(discipline),
        Err(e) => {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[utoipa::path(
    request_body = DisciplineIn,
    responses(
        (status = StatusCode::OK, description = "Successful updating of discipline by id"),
        (status = StatusCode::NOT_FOUND, description = "Discipline not found"),
    ),
    params(
        ("id" = Uuid, Path, description = "ID"),
    )
)]
#[put("/disciplines/{id}")]
async fn update_by_id(
    ctx: Data<Context>,
    id: Path<Uuid>,
    Json(discipline): Json<models::DisciplineIn>,
) -> HttpResponse {
    match ctx.db.update_discipline_name(*id, &discipline.name).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
            _ => {
                log::error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful deleting of discipline by id"),
        (status = StatusCode::NOT_FOUND, description = "Discipline not found"),
    ),
    params(
        ("id" = Uuid, Path, description = "ID"),
    )
)]
#[delete("/disciplines/{id}")]
async fn delete_by_id(ctx: Data<Context>, id: Path<Uuid>) -> HttpResponse {
    match ctx.db.delete_discipline_by_id(*id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
            _ => {
                log::error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}
