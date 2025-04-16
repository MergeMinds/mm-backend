use crate::{
    auth::
        jwt::{create_tokens, validate_token}
    ,
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
) -> HttpResponse {
    log::trace!("Received register request");

    let mut user = user_data;
    user.password = match bcrypt::hash(user.password, bcrypt::DEFAULT_COST) {
        Ok(hashed_password) => hashed_password,
        Err(_) => {
            return HttpResponse::BadRequest().body("Hashing password error");
        }
    };

    match ctx.db.add_user(user).await {
        Ok(_) => {
            return HttpResponse::Created().finish()
        },
        Err(err) => {
            log::error!("Error adding user to database: {}", err);
            return HttpResponse::InternalServerError().body("Error while adding user to database")
        }
    }
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
) -> HttpResponse {
    log::trace!("Received login request");

    let user = match ctx.db.get_user_by_creds(&creds).await {
        Ok(user) => user,
        Err(_) => {
            let _ = match bcrypt::hash(&creds.password, bcrypt::DEFAULT_COST) {
                Ok(hash) => hash,
                Err(_) => {
                    return HttpResponse::InternalServerError().body("Hashing password error")
                }
            };
            return HttpResponse::Unauthorized().finish();
        }
    };

    let utf8_hash = match std::str::from_utf8(&user.password) {
            Ok(hash) => hash,
            Err(_) => {
                return HttpResponse::InternalServerError().body("Decoding password hash error")
            }
        };

    match bcrypt::verify(&creds.password, utf8_hash) {
        Ok(true) => {
            log::trace!("User has been verified");
        }
        Ok(false) => {
            return HttpResponse::Unauthorized().finish();
        }
        Err(_) => {
            return HttpResponse::InternalServerError().body("Verifying password error")
        }
    }

    let (access_token, refresh_token) = match
        create_tokens(&ctx.config, &user.email) {
            Ok(tokens) => tokens,
            Err(_) => {
                return HttpResponse::InternalServerError().body("Creating tokens error")
            }
        };

    let cookie_to_add = |name, token| {
        Cookie::build(name, token)
            .path("/")
            .http_only(true)
            .finish()
    };

    HttpResponse::Ok()
        .cookie(cookie_to_add("access_token", access_token))
        .cookie(cookie_to_add("refresh_token", refresh_token))
        .finish()
}

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful refresh of JWT token"),
    )
)]
#[post("/refresh")]
async fn refresh(ctx: Data<Context>, req: HttpRequest) -> HttpResponse {
    let Some(cookie) = req.cookie("refresh_token") else {
        return HttpResponse::Unauthorized().finish();
    };

    let claims = match validate_token(&ctx.config, cookie.value()) {
        Ok(claims) => claims,
        Err(_) => {
            return HttpResponse::InternalServerError().finish()
        }
    };
    let (access_token, refresh_token) = match
        create_tokens(&ctx.config, &claims.sub) {
            Ok(tokens) => tokens,
            Err(_) => {
                return HttpResponse::InternalServerError().body("Error while creating tokens")
            }
        };

    let cookie_to_add = |name, token| {
        Cookie::build(name, token)
            .path("/")
            .http_only(true)
            .finish()
    };

    HttpResponse::Ok()
        .cookie(cookie_to_add("access_token", access_token))
        .cookie(cookie_to_add("refresh_token", refresh_token))
        .finish()
}

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful logout"),
    )
)]
#[post("/logout")]
async fn logout() -> HttpResponse {
    // NOTE(granatam): We cannot delete cookies, so we explicitly set its
    // expiration time to the elapsed time
    let cookie_to_delete = |name| {
        Cookie::build(name, "")
            .path("/")
            .http_only(true)
            .expires(Expiration::from(OffsetDateTime::UNIX_EPOCH))
            .finish()
    };

    HttpResponse::Ok()
        .cookie(cookie_to_delete("access_token"))
        .cookie(cookie_to_delete("refresh_token"))
        .finish()
}
