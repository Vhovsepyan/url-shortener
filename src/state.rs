use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::config::AppConfig;

pub type Store = Arc<RwLock<HashMap<String, String>>>;

#[derive(Clone)]
pub struct AppState {
    pub store: Store,
    pub config: AppConfig,
}