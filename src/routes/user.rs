use crate::{data::AppState, handlers};
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(handlers::list_users))
        .route("/{id}", get(handlers::get_user))
        .route("/create", post(handlers::create_user))
}
