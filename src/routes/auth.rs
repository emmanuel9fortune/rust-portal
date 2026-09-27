use axum::{
    routing::post,
    Router,
};

use crate::{
    handlers::auth::login,
    state::AppState,
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
}
