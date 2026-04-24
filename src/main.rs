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
use axum_prometheus::PrometheusMetricLayer; // <-- 1. Import the layer
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    config::AppConfig,
    handlers::{health, redirect, shorten},
    repository::PostgresRepository,
    state::AppState,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "url_shortener=debug,axum=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting URL shortener application...");

    let config = AppConfig::new();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (
            code VARCHAR(20) PRIMARY KEY,
            original_url TEXT NOT NULL
        )"
    )
        .execute(&pool)
        .await
        .expect("Failed to initialize database table");

    let repository = Arc::new(PostgresRepository::new(pool));

    tracing::info!("Connecting to Redis...");
    let redis_client = redis::Client::open(config.redis_url.clone()).expect("Invalid Redis URL");
    let redis_conn = redis_client
        .get_multiplexed_async_connection()
        .await
        .expect("Failed to connect to Redis");

    let state = AppState {
        repository,
        config: config.clone(),
        redis: redis_conn,
    };

    // 2. Initialize the Prometheus metric layer and the handle to read the metrics
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app = Router::new()
        .route("/health", get(health))
        .route("/shorten", post(shorten))
        .route("/{code}", get(redirect))
        // 3. Expose the metrics endpoint
        .route("/metrics", get(|| async move { metric_handle.render() }))
        // 4. Attach the middleware layer to the router
        .layer(prometheus_layer)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.server_addr)
        .await
        .unwrap();

    tracing::info!("server running on {}", config.base_url);

    axum::serve(listener, app).await.unwrap();
}