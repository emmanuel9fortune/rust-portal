use axum::{
    extract::Extension,
    Json,
};

use serde::Serialize;

use crate::{
    errors::AppError,
    middleware::auth::AuthenticatedUser,
};

#[derive(Debug, Serialize)]
pub struct ProtectedResponse {
    pub success: bool,
    pub message: String,
    pub user_id: String,
    pub role: crate::models::user::UserRole,
}

pub async fn test_protected(
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Json<ProtectedResponse>, AppError> {
    Ok(Json(ProtectedResponse {
        success: true,
        message: "Authentication successful".to_string(),
        user_id: user.user_id,
        role: user.role,
    }))
}