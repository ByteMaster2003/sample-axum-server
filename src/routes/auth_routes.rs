use crate::{data::SharedState, handlers, middleware::auth::auth_middleware};
use axum::{
    Router,
    routing::{get, post},
};

pub fn auth_routes(state: SharedState) -> Router<SharedState> {
    let protected = Router::new()
        .route("/me", get(handlers::get_user_info))
        .layer(axum::middleware::from_fn(auth_middleware))
        .with_state(state.clone());

    Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .merge(protected)
        .with_state(state.clone())
}
