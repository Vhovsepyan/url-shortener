use axum::{
    extract::{Json, Path, State},
    response::Redirect,
};

use crate::{
    errors::AppError,
    models::{ShortenRequest, ShortenResponse},
    service,
    state::AppState,
};

pub async fn health() -> &'static str {
    "OK"
}

pub async fn shorten(
    State(state): State<AppState>,
    Json(request): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, AppError> {
    let code = service::shorten_url(&state, request.url.clone()).await?;
    Ok(Json(ShortenResponse {
        code: code.clone(),
        short_url: format!("{}/{}", state.config.base_url, code),
        original_url: request.url,
    }))
}

pub async fn redirect(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Redirect, AppError> {
    let original_url = service::get_redirect(&state, &code).await?;

    Ok(Redirect::temporary(&original_url))
}