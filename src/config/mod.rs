use std::env;
// THIS CONTAINS THE APPLICATIONS IMPORTANT SETTINGS, 
// SUCH AS DATABASE CONNECTIONS AND SERVER PORTS
#[derive(Clone)]
pub struct Config{
    pub mongodb_uri: String,
    pub database_name: String,
    pub server_port: u16,
    pub frontend_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI is not set");

        let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not set");

        let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "4000".to_string())
        .parse::<u16>().expect("SERVER_PORT must be a valid number");

        let frontend_url = env::var("FRONTEND_URL")
        .expect("FRONTEND_URL is not set");

        let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET is not set");

        Self {
            mongodb_uri,
            database_name,
            server_port,
            frontend_url,
            jwt_secret,
        }
    }
}