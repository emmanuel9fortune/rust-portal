use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},Json
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    Internal,
    BadRequest(String),
    Unauthorized,
    Forbidden,
    NotFound,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                message,
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized".to_string(),
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "Forbidden".to_string(),
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "Resource not found".to_string(),
            ),
        };

        let body = Json(ErrorResponse {
            success: false,
            message,
        });

        (status, body).into_response()
    }
}