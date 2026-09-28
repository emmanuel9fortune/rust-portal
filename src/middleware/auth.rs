use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};

use crate::{
    errors::AppError,
    services::token_service::verify_access_token,
    state::AppState,
};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: String,
    pub role: crate::models::user::UserRole,
}

pub async fn authenticate(
    State(state): State<AppState>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, AppError> {
    // Get the Authorization header
    let authorization = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| {
            AppError::Unauthorized("Authorization token is required".to_string())
        })?;

    // Make sure it uses the Bearer scheme
    let token = authorization
        .strip_prefix("Bearer ")
        .ok_or_else(|| {
            AppError::Unauthorized("Invalid authorization format".to_string())
        })?;

    if token.trim().is_empty() {
        return Err(
            AppError::Unauthorized(
                "Authorization token is required".to_string()
            )
        );
    }

    // Verify the JWT
    let claims = verify_access_token(
        token,
        &state.config.jwt_secret,
    )
    .map_err(|_| {
        AppError::Unauthorized("Invalid or expired token".to_string())
    })?;

    // Store authenticated user information inside the request.
    request.extensions_mut().insert(
        AuthenticatedUser {
            user_id: claims.sub,
            role: claims.role,
        },
    );

    // Continue to the protected route
    Ok(next.run(request).await)
}