use crate::{
    auth::jwt::{create_tokens, validate_token},
    context::Context,
    errors::APIError,
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
) -> Result<HttpResponse, APIError> {
    log::trace!("Received register request");

    let mut user = user_data;
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
) -> Result<HttpResponse, APIError> {
    log::trace!("Received login request");

    let user = ctx.db.get_user_by_creds(&creds).await.map_err(|_| {
        // NOTE(t3m8ch): This line will never throw an error because it depends
        // on the second argument we have fixed. That's why unwrap is used here.
        let _ = bcrypt::hash(&creds.password, bcrypt::DEFAULT_COST).unwrap();
        APIError::WrongCredentials
    })?;

    let utf8_hash = std::str::from_utf8(&user.password)
        .map_err(|_| APIError::WrongCredentials)?;

    if !bcrypt::verify(&creds.password, utf8_hash)
        .map_err(|_| APIError::InternalServerError)?
    {
        return Err(APIError::WrongCredentials);
    }
    log::trace!("User has been verified");

    let (access_token, refresh_token) = create_tokens(&ctx.config, &user.email)
        .map_err(|_| APIError::InternalServerError)?;

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
async fn refresh(
    ctx: Data<Context>,
    req: HttpRequest,
) -> Result<HttpResponse, APIError> {
    let Some(cookie) = req.cookie("refresh_token") else {
        return Err(APIError::InvalidToken);
    };

    let claims = validate_token(&ctx.config, cookie.value())
        .map_err(|_| APIError::InvalidToken)?;

    let (access_token, refresh_token) = create_tokens(&ctx.config, &claims.sub)
        .map_err(|_| APIError::InternalServerError)?;

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
async fn logout() -> Result<HttpResponse, APIError> {
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
