use axum::{
    routing::{get, post}, 
    Router,
};

use crate::{
    handlers::{
        health::health_check,
        validation_test::validation_test,
    },
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
    .route("/health", get(health_check)) //creating the health API
    .route("/validation-test", post(validation_test))
    .with_state(state)
}