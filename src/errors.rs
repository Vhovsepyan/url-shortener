use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("invalid url")]
    InvalidUrl,
    #[error("Short code not found")]
    ShortCodeNotFound,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error), // Automatically converts sqlx errors
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::InvalidUrl => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::ShortCodeNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::DatabaseError(_) => {
                // We log the real error to the console, but hide details from the user
                eprintln!("DB Error: {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let body = Json(ErrorResponse { error: message });
        (status, body).into_response()
    }
}