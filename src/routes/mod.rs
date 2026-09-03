use axum::{
    routing::get, 
    Router,
};

use crate::{
    handlers::health::health_check,
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
    .route("/health", get(health_check)) //creating the health API
    .with_state(state)
}