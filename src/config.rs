use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub host: String,
    pub port: u16,
    pub public_api_key: String,
    pub private_api_key: String,
    pub secret_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL not set");
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap();
        let public_api_key = env::var("PUBLIC_API_KEY").expect("PUBLIC_API_KEY not set");
        let private_api_key = env::var("PRIVATE_API_KEY").expect("PRIVATE_API_KEY not set");
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not set");
        Self { database_url, redis_url, host, port, public_api_key, private_api_key, secret_key }
    }
}