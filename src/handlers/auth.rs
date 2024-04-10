use actix_web::{post, HttpResponse, Result};

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful registration"),
        (status = StatusCode::BAD_REQUEST, description = "Invalid user data"),
    )
)]
#[post("/register")]
async fn register() -> Result<HttpResponse> {
    todo!()
}

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful login"),
        (status = StatusCode::UNAUTHORIZED, description = "Incorrect username or password"),
    )
)]
#[post("/login")]
async fn login() -> Result<HttpResponse> {
    todo!()
}

#[utoipa::path(
    responses(
        (status = StatusCode::OK, description = "Successful refresh of JWT token"),
    )
)]
#[post("/refresh")]
async fn refresh() -> Result<HttpResponse> {
    todo!()
}
