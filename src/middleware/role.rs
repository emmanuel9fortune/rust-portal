use axum::{
    extract::Extension,
    http::Request,
    middleware::Next,
    response::Response,
};

use crate::{
    errors::AppError,
    middleware::auth::AuthenticatedUser,
    models::permissions::Permission,
    services::permission_service::has_permission,
};

pub fn check_permission(
    user: &AuthenticatedUser,
    permission: &Permission,
) -> Result<(), AppError> {
    if !has_permission(&user.role, permission) {
        return Err(
            AppError::Forbidden(
                "You do not have permission to access this resource".to_string(),
            )
        );
    }

    Ok(())
}

pub async fn require_permission(
    Extension(user): Extension<AuthenticatedUser>,
    Extension(permission): Extension<Permission>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, AppError> {
    check_permission(
        &user,
        &permission,
    )?;

    Ok(next.run(request).await)
}