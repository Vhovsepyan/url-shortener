use rand::{distributions::Alphanumeric, Rng};
use crate::{errors::AppError, state::AppState};

// Added async
pub async fn shorten_url(state: &AppState, original_url: String) -> Result<String, AppError> {
    validate_url(&original_url)?;

    let length = state.config.code_length;
    // Added .await
    let code = insert_with_unique_code(state, original_url, length).await?;

    Ok(code)
}

// Added async
pub async fn get_redirect(state: &AppState, code: &str) -> Result<String, AppError> {
    state
        .repository
        .find_by_code(code)
        .await? // Added .await
        .ok_or(AppError::ShortCodeNotFound)
}

fn validate_url(url: &str) -> Result<(), AppError> {
    let trimmed = url.trim();
    if trimmed.is_empty() || !(trimmed.starts_with("http://") || trimmed.starts_with("https://")) {
        return Err(AppError::InvalidUrl);
    }
    Ok(())
}

// Added async
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