use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error)]
pub enum APIError {
    NotFound,
    WrongCredentials,
    InvalidToken,
    InternalServer,
}

impl From<sqlx::Error> for APIError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => APIError::NotFound,
            _ => APIError::InternalServer,
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
            APIError::NotFound => StatusCode::NOT_FOUND,
            APIError::WrongCredentials => StatusCode::UNAUTHORIZED,
            APIError::InvalidToken => StatusCode::UNAUTHORIZED,
            APIError::InternalServer => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(match self {
            APIError::NotFound => ErrorBody { error: "NOT_FOUND" },
            APIError::WrongCredentials => ErrorBody {
                error: "WRONG_CREDENTIALS",
            },
            APIError::InvalidToken => ErrorBody {
                error: "INVALID_TOKEN",
            },
            APIError::InternalServer => ErrorBody {
                error: "INTERNAL_SERVER",
            },
        })
    }
}
