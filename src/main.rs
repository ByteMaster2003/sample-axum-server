use backend_ax::{
    data::{AppState, env_config},
    repository, routes,
};
use std::sync::Arc;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load Environment Vars
    let cfg = env_config();

    // Initialize logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    // Connect Database
    let db = repository::establish_connection(&cfg.database_url, &cfg.database_name).await;
    tracing::info!("✅ Successfully connected to MongoDB");

    // Set App State
    let state = Arc::new(AppState { db });

    // Create App Routes
    let app = routes::create_router(state);

    // Bind TCP server
    let addr = format!("0.0.0.0:{}", cfg.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("🚀 Server is running on {}", addr);

    // Serve The App with axum
    axum::serve(listener, app).await.unwrap();
}
