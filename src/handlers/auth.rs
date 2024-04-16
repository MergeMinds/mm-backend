use crate::{
    db::core::PgConnection,
    models::{SignInCredentials, SignUpCredentials},
    utils::{error::Result, jwt},
};

use actix_web::{
    cookie::{Cookie, Expiration},
    post,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use time::{Duration, OffsetDateTime};

#[utoipa::path(
    responses(
        (status = StatusCode::CREATED, description = "Successful registration"),
        (status = StatusCode::BAD_REQUEST, description = "Invalid user data"),
    )
)]
#[post("/register")]
async fn register(
    db: Data<PgConnection>,
    Json(user_data): Json<SignUpCredentials>,
) -> Result<HttpResponse> {
    log::trace!("Received register request");

    let mut user = user_data;
    user.password = bcrypt::hash(user.password, bcrypt::DEFAULT_COST)?;

    db.add_user(user).await?;

    Ok(HttpResponse::Created().into())
}

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful login"),
        (status = StatusCode::UNAUTHORIZED, description = "Incorrect username or password"),
    )
)]
#[post("/login")]
async fn login(
    db: Data<PgConnection>,
    Json(creds): Json<SignInCredentials>,
) -> Result<HttpResponse> {
    log::trace!("Received login request");

    if let Ok(user) = db.verify_creds(creds).await {
        log::trace!("User has been verified");

        let access_token = jwt::create_token(&user.email, user.role.clone())?;
        let refresh_token = jwt::create_token(&user.email, user.role)?;

        let cookie_to_add = |name, token| {
            Cookie::build(name, token)
                .path("/")
                .http_only(true)
                .finish()
        };
        Ok(HttpResponse::Ok()
            .cookie(cookie_to_add("access_token", access_token))
            .cookie(cookie_to_add("refresh_token", refresh_token))
            .finish())
    } else {
        Ok(HttpResponse::Unauthorized().into())
    }
}

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful refresh of JWT token"),
    )
)]
#[post("/refresh")]
async fn refresh(req: HttpRequest) -> Result<HttpResponse> {
    if let Some(cookie) = req.cookie("refresh_token") {
        let claims = jwt::validate_token(cookie.value())?;
        let access_token = jwt::create_token(&claims.sub, claims.role.clone())?;
        let refresh_token = jwt::create_token(&claims.sub, claims.role)?;

        let cookie_to_add = |name, token| {
            Cookie::build(name, token)
                .path("/")
                .http_only(true)
                .finish()
        };

        Ok(HttpResponse::Ok()
            .cookie(cookie_to_add("access_token", access_token))
            .cookie(cookie_to_add("refresh_token", refresh_token))
            .finish())
    } else {
        Ok(HttpResponse::Unauthorized().into())
    }
}

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful logout"),
    )
)]
#[post("/logout")]
async fn logout() -> Result<HttpResponse> {
    // NOTE(granatam): We cannot delete cookies, so we explicitly set its
    // expiration time to the elapsed time
    let cookie_to_delete = |name| {
        Cookie::build(name, "")
            .path("/")
            .http_only(true)
            .expires(Expiration::from(
                OffsetDateTime::now_utc() - Duration::days(1),
            ))
            .finish()
    };

    Ok(HttpResponse::Ok()
        .cookie(cookie_to_delete("access_token"))
        .cookie(cookie_to_delete("refresh_token"))
        .finish())
}
