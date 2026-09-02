mod config;
mod database;
mod handlers;
mod routes;
mod state;

use std::env;

use tokio::net::TcpListener;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI is not set");
    let database_name = env::var("DATABASE_NAME").expect("MONGODB_NAME is not set");

    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "4000".to_string());

    let database = database::connect(
        &mongodb_uri,
        &database_name,
    ).await.expect("Failed to connect to MongoDB");

    let state = AppState{
        database,
    };

    let app = routes::create_router(state);

    let address = format!("localhost:{}", server_port);

    let listener = TcpListener::bind(&address).await.expect("Failed to bind server");

    println!("School Portal API running at http://{}", address);

    axum::serve(listener, app).await.expect("Server error");
}