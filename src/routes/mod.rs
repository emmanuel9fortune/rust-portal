use axum::{
    middleware,
    routing::{get, post},
    Extension,
    Router,
};

use crate::{
    handlers::{
        auth::{
            bootstrap_super_admin,
            login,
            me,
        },
        protected::test_protected,
    },
    middleware::{
        auth::authenticate,
        role::require_permission,
    },
    models::permissions::Permission,
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    let admin_routes = Router::new()
        .route("/admin/test", get(test_protected))
        .layer(
            middleware::from_fn(require_permission)
        )
        .layer(
            Extension(Permission::UsersRead)
        );

    let protected_routes = Router::new()
        .route("/auth/me", get(me))
        .merge(admin_routes)
        .layer(
            middleware::from_fn_with_state(
                state.clone(),
                authenticate,
            )
        );

    Router::new()
        .route("/auth/login", post(login))
        .route(
            "/auth/bootstrap-super-admin",
            post(bootstrap_super_admin),
        )
        .merge(protected_routes)
        .with_state(state)
}