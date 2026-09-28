use axum::{
    extract::State,
    Json,
};

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    errors::AppError,
    models::user::{UserRole, UserStatus},
    services::{
        password_service::verify_password,
        token_service::create_access_token,
        user_service::find_user_by_email,
    },
    state::AppState,
};

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct BootstrapSuperAdminRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub access_token: String,
    pub user: LoginUser,
}

#[derive(Debug, Serialize)]
pub struct LoginUser {
    pub id: String,
    pub email: String,
    pub role: UserRole,
    pub status: UserStatus,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {

    // 1. Validate the request
    payload
        .validate()
        .map_err(|error| AppError::BadRequest(error.to_string()))?;

    // 2. Normalize the email
    let email = payload.email.trim().to_lowercase();

    // 3. Find the user by email
    let user = find_user_by_email(
        &state.database,
        &email,
    )
    .await
    .map_err(|_| AppError::Internal)?;

    // 4. Email was not found
    let user = user.ok_or(
        AppError::NotFound("Email not found".to_string())
    )?;

    // 5. Check account status
    match user.status {
        UserStatus::Active => {}

        UserStatus::Suspended | UserStatus::Disabled => {
            return Err(
                AppError::Unauthorized("Account is not active".to_string())
            );
        }
    }

    // 6. Verify the password
    let password_correct = verify_password(
        &payload.password,
        &user.password_hash,
    )
    .map_err(|_| AppError::Internal)?;

    // 7. Password is incorrect
    if !password_correct {
        return Err(
            AppError::Unauthorized("Incorrect password".to_string())
        );
    }

    let access_token = create_access_token(
        user.id.to_hex(),
        user.role.clone(),
        &state.config.jwt_secret,
    )
    .map_err(|_| AppError::Internal)?;

    // 8. Login successful
    let response = LoginResponse {
        success: true,
        message: "Login successful".to_string(),
        access_token,
        user: LoginUser {
            id: user.id.to_hex(),
            email: user.email,
            role: user.role,
            status: user.status,
        },
    };

    Ok(Json(response))
}

pub async fn bootstrap_super_admin(
    State(state): State<AppState>,
    Json(payload): Json<BootstrapSuperAdminRequest>,
) -> Result<Json<LoginResponse>, AppError> {

    payload
        .validate()
        .map_err(|error| AppError::BadRequest(error.to_string()))?;

    // Check whether a Super Admin already exists
    let users = state
        .database
        .collection::<crate::models::user::User>("users");

    let existing_super_admin = users
        .find_one(
            mongodb::bson::doc! {
                "role": "SuperAdmin"
            }
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if existing_super_admin.is_some() {
        return Err(
            AppError::Forbidden(
                "Super Admin has already been created".to_string()
            )
        );
    }

    // Create the first Super Admin using the existing user service
    let request = crate::services::user_service::CreateUserRequest {
        email: payload.email,
        password: payload.password,
        role: crate::models::user::UserRole::SuperAdmin,
    };

    let user = crate::services::user_service::create_user(
        &state.database,
        request,
    )
    .await
    .map_err(|error| {

        if error.contains("already exists") {
            AppError::BadRequest(error)
        } else {
            AppError::Internal
        }
    })?;

    let access_token = create_access_token(
        user.id.to_hex(),
        user.role.clone(),
        &state.config.jwt_secret,
    )
    .map_err(|_| AppError::Internal)?;

    Ok(Json(LoginResponse {
        success: true,
        message: "Super Admin created successfully".to_string(),
        access_token,
        user: LoginUser {
            id: user.id.to_hex(),
            email: user.email,
            role: user.role,
            status: user.status,
        },
    }))
}
