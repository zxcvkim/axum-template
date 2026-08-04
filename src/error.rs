use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("not found")]
    NotFound,

    #[error("internal server error")]
    Internal(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: &'static str,
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        use AppError::*;
        match self {
            NotFound => StatusCode::NOT_FOUND,
            Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> &'static str {
        use AppError::*;
        match self {
            NotFound => "NOT_FOUND",
            Internal(_) => "INTERNAL_ERROR",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match &self {
            AppError::NotFound => tracing::warn!("request failed: not found"),
            AppError::Internal(e) => tracing::error!(error = ?e, "request failed: internal error"),
            _ => {}
        }

        let status = self.status_code();
        let code = self.error_code();

        let body = Json(ErrorResponse { code });

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
