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
            server_addr: "127.0.0.1:3000".to_string(),
            base_url: "http://127.0.0.1:3000".to_string(),
            code_length: 6,
            database_url: "postgres://root:root@localhost:5432/url_shortener".to_string(),
        }
    }
}