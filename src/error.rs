use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde::Serialize;

/// Represents the complete set of error codes the API may produce.
#[derive(Serialize, Clone, Debug, Display, Error)]
// NOTE(evgenymng): Intentionally not making it `Copy`, because we
// will probably need to store some additional information in those variants
// in the future.
pub enum ApiError {
    #[display("wrong_credentials")]
    WrongCredentials,
    #[display("invlid_token")]
    InvalidToken,
    #[display("internal")]
    Internal,
}

/// A convenient alias for the API handler's return type.
pub type ApiResult = Result<HttpResponse, ApiError>;

impl From<sqlx::Error> for ApiError {
    fn from(_: sqlx::Error) -> Self {
        ApiError::Internal
    }
}

/// Structured error's body. If the API call fails with an error,
/// an object with this structure is sent as a response.
#[derive(Serialize)]
struct ErrorBody {
    pub error: ApiError,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::WrongCredentials => StatusCode::UNAUTHORIZED,
            ApiError::InvalidToken => StatusCode::UNAUTHORIZED,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorBody {
            error: self.clone(),
        })
    }
}
