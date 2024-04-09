use actix_web::{post, HttpResponse, Result};

#[post("/register")]
async fn register() -> Result<HttpResponse> {
    todo!()
}

#[post("/login")]
async fn login() -> Result<HttpResponse> {
    todo!()
}

#[post("/refresh")]
async fn refresh() -> Result<HttpResponse> {
    todo!()
}
