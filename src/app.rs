use axum::{routing::get, Router};

use crate::{routes, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(routes::health::health_check))
        .with_state(state)
}
