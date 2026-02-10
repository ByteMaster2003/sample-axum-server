use dotenvy::dotenv;
use std::env;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct EnvConfig {
    pub database_url: String,
    pub database_name: String,
    pub jwt_secret: String,
    pub port: u16,
}

pub static APP_CONFIG: OnceLock<EnvConfig> = OnceLock::new();

pub fn env_config() -> &'static EnvConfig {
    APP_CONFIG.get_or_init(|| {
        dotenv().ok();

        EnvConfig {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            database_name: env::var("DATABASE_NAME").expect("DATABASE_NAME must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a number"),
        }
    })
}

#[derive(Clone)]
pub struct AppState {
    pub db: mongodb::Database,
}

// We wrap it in Arc so it can be cloned cheaply for every thread/request
pub type SharedState = std::sync::Arc<AppState>;
