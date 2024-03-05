use dotenvy::dotenv;
use std::env;

pub struct ServerConfig {
    pub api_host: String,
    pub api_port: u16,
}

impl ServerConfig {
    pub fn new() -> Result<Self, env::VarError> {
        // sets env vars from .env file
        // good for dev but for prod, opt for setting the vars directly
        dotenv().ok();

        let api_host = env::var("API_HOST").unwrap_or_else(|_| "localhost".to_string());
        let api_port = env::var("API_PORT").unwrap_or_else(|_| "5432".to_string())
            .trim()
            .parse()
            .expect("Not an integer");

        Ok(ServerConfig {api_host, api_port})
    }
}
