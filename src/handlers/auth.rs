use crate::{
    models::{SignInCredentials, SignUpCredentials},
    utils::core::Context,
    utils::{
        core::Result,
        jwt::{create_token, validate_token, TokenType},
    },
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

    if let Ok(user) = ctx.db.verify_creds(creds).await {
        log::trace!("User has been verified");

        let access_token = create_token(
            &ctx.config,
            &user.email,
            user.role.clone(),
            TokenType::AccessToken,
        )?;
        let refresh_token = create_token(
            &ctx.config,
            &user.email,
            user.role,
            TokenType::RefreshToken,
        )?;

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
async fn refresh(ctx: Data<Context>, req: HttpRequest) -> Result<HttpResponse> {
    if let Some(cookie) = req.cookie("refresh_token") {
        let claims = validate_token(&ctx.config, cookie.value())?;
        let access_token = create_token(
            &ctx.config,
            &claims.sub,
            claims.role.clone(),
            TokenType::AccessToken,
        )?;
        let refresh_token = create_token(
            &ctx.config,
            &claims.sub,
            claims.role,
            TokenType::RefreshToken,
        )?;

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
