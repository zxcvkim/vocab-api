use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
struct Params {
    first: Option<char>,
    min_chars: Option<usize>,
    max_chars: Option<usize>,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(handler))
}

async fn handler(State(state): State<AppState>, Query(params): Query<Params>) -> Json<Vec<String>> {
    let result = state
        .vocab
        .filter(params.first, params.min_chars, params.max_chars)
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    Json(result)
}
