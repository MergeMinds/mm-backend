use crate::{
    auth::{
        jwt::{create_tokens, validate_token},
        Error, Result,
    },
    context::Context,
    models::{SignInCredentials, SignUpCredentials},
};

use actix_web::{
    cookie::{Cookie, Expiration},
    post,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use time::OffsetDateTime;

#[utoipa::path(
    responses(
        (status = StatusCode::CREATED, description = "Successful registration"),
        (status = StatusCode::BAD_REQUEST, description = "Invalid user data"),
    )
)]
#[post("/register")]
async fn register(
    ctx: Data<Context>,
    Json(user_data): Json<SignUpCredentials>,
) -> Result<HttpResponse> {
    log::trace!("Received register request");

    let mut user = user_data;
    user.password = bcrypt::hash(user.password, bcrypt::DEFAULT_COST)?;

    ctx.db.add_user(user).await?;

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
    ctx: Data<Context>,
    Json(creds): Json<SignInCredentials>,
) -> Result<HttpResponse> {
    log::trace!("Received login request");

    let Ok(user) = ctx.db.get_user_by_creds(&creds).await else {
        let _ = bcrypt::hash(&creds.password, bcrypt::DEFAULT_COST)?;
        return Ok(HttpResponse::Unauthorized().finish());
    };

    let utf8_hash =
        std::str::from_utf8(&user.password).map_err(|_| Error::Auth)?;

    if !bcrypt::verify(&creds.password, utf8_hash)? {
        return Err(Error::Auth);
    }
    log::trace!("User has been verified");

    let (access_token, refresh_token) =
        create_tokens(&ctx.config, &user.email)?;

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
}

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful refresh of JWT token"),
    )
)]
#[post("/refresh")]
async fn refresh(ctx: Data<Context>, req: HttpRequest) -> Result<HttpResponse> {
    let Some(cookie) = req.cookie("refresh_token") else {
        return Ok(HttpResponse::Unauthorized().finish());
    };

    let claims = validate_token(&ctx.config, cookie.value())?;
    let (access_token, refresh_token) =
        create_tokens(&ctx.config, &claims.sub)?;

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
            .expires(Expiration::from(OffsetDateTime::UNIX_EPOCH))
            .finish()
    };

    Ok(HttpResponse::Ok()
        .cookie(cookie_to_delete("access_token"))
        .cookie(cookie_to_delete("refresh_token"))
        .finish())
}
