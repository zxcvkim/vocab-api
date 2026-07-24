mod words;

use axum::Router;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().nest("/words", words::router())
}
