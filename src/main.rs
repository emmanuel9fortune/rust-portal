// THESE ARE MODUELS OF THE APPLICATION
mod errors;
mod logging;
mod config;
mod database;
mod handlers;
mod routes;
mod state;
mod middleware;

use axum::{
    http::{
        header::{HeaderName, HeaderValue},
        Method,
    },
    extract::DefaultBodyLimit,
};
use tower_http::{
    cors::CorsLayer,
    set_header::SetResponseHeaderLayer,
};
use tower_governor::{
    governor::GovernorConfigBuilder,
    GovernorLayer,
};

use std::env;

use tokio::net::TcpListener;


use crate::{
    config::Config,
    state::AppState,
};

#[tokio::main]
async fn main() {
    logging::init();
    dotenvy::dotenv().ok();

    let config = Config::from_env(); // CALLING THE CONFIG FUNCTION THAT CONTAINS THE MONGODB URI, DATABASE NAME, SERVERPORT

    // CALLING OUR MONGODB FUNCTION WE CREATED IN DATABASE/MOD.RS
    let database = database::connect(
        &config.mongodb_uri,
        &config.database_name,
    ).await.expect("Failed to connect to MongoDB");

    let state = AppState{
        database,
        config: config.clone(),
    };

    let frontend_origin = config.frontend_url.parse::<HeaderValue>().expect("Invalid CORS origin");

    let cors = CorsLayer::new()
    .allow_origin(frontend_origin)
    .allow_methods([
        axum::http::Method::GET,
        axum::http::Method::POST,
        axum::http::Method::PUT,
        axum::http::Method::PATCH,
        axum::http::Method::DELETE,
    ])
    .allow_headers([
        axum::http::header::CONTENT_TYPE,
        axum::http::header::AUTHORIZATION,
    ]);

    let governor_config = GovernorConfigBuilder::default()
    .per_second(2)
    .burst_size(10)
    .finish()
    .expect("Failed to create rate limiter");

    let app = routes::create_router(state)
    .layer(DefaultBodyLimit::max(1 * 1024 * 1024))
    .layer(GovernorLayer::new(governor_config))
    .layer(cors)
    .layer(
        SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        )
    )
    .layer(
        SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        )
    )
    .layer(
        SetResponseHeaderLayer::overriding(
            HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("no-referrer"),
        )
    )
    .layer(
        SetResponseHeaderLayer::overriding(
            HeaderName::from_static("permissions-policy"),
            HeaderValue::from_static("camera=(), microphone=(), geoloction=()"),
        )
    );
    

    let address = format!("localhost:{}", config.server_port);

    let listener = TcpListener::bind(&address).await.expect("Failed to bind server");

    tracing::info!(address = %address, "School Portal API running");

    axum::serve(
        listener, 
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(), 
    ).await.expect("Server error");
}