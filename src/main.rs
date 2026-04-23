mod config;
mod errors;
mod handlers;
mod models;
mod state;
mod service;

use axum::{
    routing::{get, post},
    Router,
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    config::AppConfig,
    handlers::{health, redirect, shorten},
    state::AppState,
};

#[tokio::main]
async fn main() {
    // Changed: configuration is now created centrally.
    let config = AppConfig::new();

    // Changed: AppState now stores both the in-memory store and config.
    let state = AppState {
        store: Arc::new(RwLock::new(HashMap::new())),
        config: config.clone(),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/shorten", post(shorten))
        .route("/{code}", get(redirect))
        .with_state(state);

    // Changed: bind address now comes from configuration.
    let listener = tokio::net::TcpListener::bind(&config.server_addr)
        .await
        .unwrap();

    println!("server running on {}", config.base_url);

    axum::serve(listener, app)
        .await
        .unwrap();
}