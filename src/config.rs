use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub server_addr: String,
    pub base_url: String,
    pub code_length: usize,
    pub database_url: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            // Provide a fallback if the environment variable is missing
            server_addr: env::var("SERVER_ADDR")
                .unwrap_or_else(|_| "127.0.0.1:3000".to_string()),

            base_url: env::var("BASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string()),

            // We have to parse the string into a usize
            code_length: env::var("CODE_LENGTH")
                .unwrap_or_else(|_| "6".to_string())
                .parse()
                .expect("CODE_LENGTH must be a valid number"),

            // Fail fast if the database URL is completely missing
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL environment variable must be set"),
        }
    }
}