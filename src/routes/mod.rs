mod test;

use axum::Router;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().nest("/test", test::router())
}
