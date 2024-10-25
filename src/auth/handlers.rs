use crate::{
    auth::jwt::{create_tokens, validate_token},
    context::Context,
    error::{ApiError, ApiResult},
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
) -> ApiResult {
    let mut user = user_data;
    // NOTE(evgenymng): This call will never end with an error, because it can
    // only produce one, when the cost is invalid, which we cannot possibly have.
    user.password = bcrypt::hash(user.password, bcrypt::DEFAULT_COST).unwrap();

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
) -> ApiResult {
    let user = ctx.db.get_user_by_creds(&creds).await.map_err(|_| {
        // NOTE(t3m8ch): This call will never end with an error, because it can only
        // produce one, when the cost is invalid, which we cannot possibly have.
        // NOTE(evgenymng): Masquerade as if the user exists and spend time
        // calculating hash.
        let _ = bcrypt::hash(&creds.password, bcrypt::DEFAULT_COST).unwrap();
        ApiError::WrongCredentials
    })?;

    let utf8_hash = std::str::from_utf8(&user.password)
        .map_err(|_| ApiError::WrongCredentials)?;

    if !bcrypt::verify(&creds.password, utf8_hash)
        .map_err(|_| ApiError::Internal)?
    {
        return Err(ApiError::WrongCredentials);
    }

    let (access_token, refresh_token) = create_tokens(&ctx.config, &user.email)
        .map_err(|_| ApiError::Internal)?;

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
async fn refresh(ctx: Data<Context>, req: HttpRequest) -> ApiResult {
    let cookie = req.cookie("refresh_token").ok_or(ApiError::InvalidToken)?;

    let claims = validate_token(&ctx.config, cookie.value())
        .map_err(|_| ApiError::InvalidToken)?;

    let (access_token, refresh_token) = create_tokens(&ctx.config, &claims.sub)
        .map_err(|_| ApiError::Internal)?;

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
async fn logout() -> ApiResult {
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
