use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error)]
pub enum ApiError {
    NotFound,
    WrongCredentials,
    InvalidToken,
    InternalServerError,
}

pub type ApiResult = Result<HttpResponse, ApiError>;

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => ApiError::NotFound,
            _ => ApiError::InternalServerError,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorBody {
    pub error: String,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::WrongCredentials => StatusCode::UNAUTHORIZED,
            ApiError::InvalidToken => StatusCode::UNAUTHORIZED,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(ErrorBody {
            error: format!("{}", self),
        })
    }
}
