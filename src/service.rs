use rand::{distributions::Alphanumeric, Rng};

use crate::{
    errors::AppError,
    state::AppState,
};

/// Handles the business logic for shortening a URL.
pub fn shorten_url(state: &AppState, original_url: String) -> Result<String, AppError> {
    validate_url(&original_url)?;

    let length = state.config.code_length;
    let code = insert_with_unique_code(state, original_url, length);

    Ok(code)
}

/// Handles the business logic for looking up a redirect.
pub fn get_redirect(state: &AppState, code: &str) -> Result<String, AppError> {
    let store = state.store.read().unwrap();

    // We clone the string so we don't hold a reference to the data inside the RwLock.
    // This prevents keeping the read lock open longer than necessary.
    store.get(code).cloned().ok_or(AppError::ShortCodeNotFound)
}

// Below are our private helper functions, completely hidden from the rest of the app.

fn validate_url(url: &str) -> Result<(), AppError> {
    let trimmed = url.trim();

    if trimmed.is_empty() {
        return Err(AppError::InvalidUrl);
    }

    if !(trimmed.starts_with("http://") || trimmed.starts_with("https://")) {
        return Err(AppError::InvalidUrl);
    }

    Ok(())
}

fn insert_with_unique_code(state: &AppState, original_url: String, length: usize) -> String {
    let mut store = state.store.write().unwrap();

    loop {
        let code = generate_code(length);

        if !store.contains_key(&code) {
            store.insert(code.clone(), original_url);
            return code;
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