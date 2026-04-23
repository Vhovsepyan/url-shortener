use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use sqlx::PgPool;
use async_trait::async_trait; // 1. Import the macro

use crate::errors::AppError;

// 2. Attach it to the trait
#[async_trait]
pub trait UrlRepository: Send + Sync {
    async fn save(&self, code: String, original_url: String) -> Result<bool, AppError>;
    async fn find_by_code(&self, code: &str) -> Result<Option<String>, AppError>;
}

pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// 3. Attach it to the Postgres implementation
#[async_trait]
impl UrlRepository for PostgresRepository {
    async fn save(&self, code: String, original_url: String) -> Result<bool, AppError> {
        let result = sqlx::query!(
            "INSERT INTO urls (code, original_url) VALUES ($1, $2) ON CONFLICT (code) DO NOTHING",
            code,
            original_url
        )
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn find_by_code(&self, code: &str) -> Result<Option<String>, AppError> {
        let record = sqlx::query!(
            "SELECT original_url FROM urls WHERE code = $1",
            code
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(record.map(|r| r.original_url))
    }
}

pub struct InMemoryRepository {
    store: Arc<RwLock<HashMap<String, String>>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

// 4. Attach it to the InMemory implementation
#[async_trait]
impl UrlRepository for InMemoryRepository {
    async fn save(&self, code: String, original_url: String) -> Result<bool, AppError> {
        let mut store = self.store.write().unwrap();
        if store.contains_key(&code) {
            return Ok(false);
        }
        store.insert(code, original_url);
        Ok(true)
    }

    async fn find_by_code(&self, code: &str) -> Result<Option<String>, AppError> {
        let store = self.store.read().unwrap();
        Ok(store.get(code).cloned())
    }
}