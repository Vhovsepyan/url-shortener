use axum::{
    extract::{Json, Path, State},
    response::Redirect,
};
use rand::{distributions::Alphanumeric, Rng};

use crate::{
    errors::AppError,
    models::{ShortenRequest, ShortenResponse},
    state::AppState,
};

pub async fn health() -> &'static str {
    "OK"
}

pub async fn shorten(
    State(state): State<AppState>,
    Json(request): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, AppError> {
    validate_url(&request.url)?;

    let original_url = request.url;

    let code = insert_with_unique_code(&state, original_url.clone(), state.config.code_length);

    Ok(Json(ShortenResponse {
        code: code.clone(),
        short_url: format!("{}/{}", state.config.base_url, code),
        original_url,
    }))
}

pub async fn redirect(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Redirect, AppError> {
    let store = state.store.read().unwrap();

    if let Some(original_url) = store.get(&code) {
        Ok(Redirect::temporary(original_url))
    } else {
        Err(AppError::ShortCodeNotFound)
    }
}

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