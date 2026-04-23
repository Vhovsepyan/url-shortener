use std::sync::Arc;

use crate::{config::AppConfig, repository::UrlRepository};

#[derive(Clone)]
pub struct AppState {
    // We now use dynamic dispatch (`dyn`) to point to anything that implements the trait
    pub repository: Arc<dyn UrlRepository>,
    pub config: AppConfig,
}