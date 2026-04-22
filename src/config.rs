#[derive(Clone)]
pub struct AppConfig {
    pub server_addr: String,
    pub base_url: String,
    pub code_length: usize,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            server_addr: "127.0.0.1:3000".to_string(),
            base_url: "http://127.0.0.1:3000".to_string(),
            code_length: 6,
        }
    }
}