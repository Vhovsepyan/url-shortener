mod config;
mod errors;
mod handlers;
mod models;
mod repository;
mod service;
mod state;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

use crate::{
    config::AppConfig,
    handlers::{health, redirect, shorten},
    repository::PostgresRepository, // Changed from InMemoryRepository
    state::AppState,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = AppConfig::new();

    // 1. Establish a connection pool to Postgres
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres");

    // 2. Automatically create the table if it doesn't exist (great for development)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (
            code VARCHAR(20) PRIMARY KEY,
            original_url TEXT NOT NULL
        )"
    )
        .execute(&pool)
        .await
        .expect("Failed to initialize database table");

    // 3. Inject the Postgres repository
    let repository = Arc::new(PostgresRepository::new(pool));

    let state = AppState {
        repository,
        config: config.clone(),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/shorten", post(shorten))
        .route("/{code}", get(redirect))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.server_addr)
        .await
        .unwrap();

    println!("server running on {}", config.base_url);

    axum::serve(listener, app).await.unwrap();
}