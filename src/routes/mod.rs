mod auth_routes;

use crate::data::SharedState;
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello Rust" }))
        .nest("/api/auth", auth_routes::auth_routes(state.clone()))
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone())
}
