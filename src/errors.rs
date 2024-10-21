use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error)]
pub enum APIError {
    NotFoundError,
    WrongCredentialsError,
    InvalidTokenError,
    InternalServerError,
}

impl From<sqlx::Error> for APIError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => APIError::NotFoundError,
            _ => APIError::InternalServerError,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorBody {
    pub error: &'static str,
}

impl ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        match self {
            APIError::NotFoundError => StatusCode::NOT_FOUND,
            APIError::WrongCredentialsError => StatusCode::UNAUTHORIZED,
            APIError::InvalidTokenError => StatusCode::UNAUTHORIZED,
            APIError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(match self {
            APIError::NotFoundError => ErrorBody { error: "NOT_FOUND" },
            APIError::WrongCredentialsError => ErrorBody {
                error: "WRONG_CREDENTIALS",
            },
            APIError::InvalidTokenError => ErrorBody {
                error: "INVALID_TOKEN",
            },
            APIError::InternalServerError => ErrorBody {
                error: "INTERNAL_SERVER",
            },
        })
    }
}
