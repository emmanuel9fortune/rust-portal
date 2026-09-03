// THESE ARE MODUELS OF THE APPLICATION
mod errors;
mod config;
mod database;
mod handlers;
mod routes;
mod state;

use axum::http::HeaderValue;
use tower_http::cors::CorsLayer;

use std::env;

use tokio::net::TcpListener;

use crate::{
    config::Config,
    state::AppState,
};

#[tokio::main]
async fn main() {
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

    let fontend_origin = config.frontend_url.parse::<HeaderValue>().expect("Invalid CORS origin");

    let cors = CorsLayer::new()
    .allow_origin(fontend_origin)
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

    let app = routes::create_router(state).layer(cors);

    let address = format!("localhost:{}", config.server_port);

    let listener = TcpListener::bind(&address).await.expect("Failed to bind server");

    println!("School Portal API running at http://{}", address);

    axum::serve(listener, app).await.expect("Server error");
}