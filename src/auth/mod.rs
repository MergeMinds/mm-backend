pub mod handlers;
mod jwt;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("auth error")]
    Auth,
}
impl actix_web::error::ResponseError for Error {}

pub type Result<T> = std::result::Result<T, Error>;
