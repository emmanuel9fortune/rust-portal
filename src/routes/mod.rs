use axum::{
    routing::{get, post}, 
    Router,
};

use crate::{
    handlers::auth::{
        bootstrap_super_admin,
        login,
    },
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
    .route("/auth/login", post(login))
    .route("/auth/bootstrap-super-admin", post(bootstrap_super_admin))
    .with_state(state)
}