use rand::{distributions::Alphanumeric, Rng};
use tracing::{info, instrument}; // Import tracing macros

use crate::{errors::AppError, state::AppState};

// This macro will automatically log whenever this function is called,
// including the `original_url` argument! We skip logging the `state` object because it's huge.
#[instrument(skip(state))]
pub async fn shorten_url(state: &AppState, original_url: String) -> Result<String, AppError> {
    validate_url(&original_url)?;

    let length = state.config.code_length;
    let code = insert_with_unique_code(state, original_url.clone(), length).await?;

    info!("Successfully shortened URL to code: {}", code); // Manual info log

    Ok(code)
}

#[instrument(skip(state))]
pub async fn get_redirect(state: &AppState, code: &str) -> Result<String, AppError> {
    let result = state.repository.find_by_code(code).await?;

    match result {
        Some(url) => {
            info!("Found redirect for code: {}", code);
            Ok(url)
        }
        None => {
            tracing::warn!("Redirect not found for code: {}", code); // Warning log
            Err(AppError::ShortCodeNotFound)
        }
    }
}

fn validate_url(url: &str) -> Result<(), AppError> {
    let trimmed = url.trim();
    if trimmed.is_empty() || !(trimmed.starts_with("http://") || trimmed.starts_with("https://")) {
        return Err(AppError::InvalidUrl);
    }
    Ok(())
}

async fn insert_with_unique_code(
    state: &AppState,
    original_url: String,
    length: usize,
) -> Result<String, AppError> {
    loop {
        let code = generate_code(length);
        // Added .await
        let inserted = state.repository.save(code.clone(), original_url.clone()).await?;

        if inserted {
            return Ok(code);
        }
    }
}

fn generate_code(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}