use axum::{Router, extract::State, routing::get};

use crate::{AppError, AppResult, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ping", get(handle_ping))
        .route("/state", get(handle_state))
        .route("/error", get(handle_error))
}

async fn handle_ping() -> &'static str {
    "pong"
}

async fn handle_state(State(state): State<AppState>) -> String {
    state.config.addr.to_string()
}

async fn handle_error() -> AppResult<()> {
    Err(AppError::NotFound)
}
