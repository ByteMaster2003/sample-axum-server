use axum::{Json, http, response};

type MongoError = mongodb::error::Error;

pub enum AppError {
    InvalidPayload(String),
    Conflict(String),
    DatabaseError(MongoError),
    NotFound(String),
    Unauthorized,
    Internal,
}

impl response::IntoResponse for AppError {
    fn into_response(self) -> response::Response {
        let (status, msg) = match self {
            Self::InvalidPayload(m) => (http::StatusCode::BAD_REQUEST, m),
            Self::Conflict(m) => (http::StatusCode::CONFLICT, m),
            Self::NotFound(m) => (http::StatusCode::NOT_FOUND, m),
            Self::Unauthorized => (http::StatusCode::UNAUTHORIZED, "Access Denied".into()),
            _ => (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Error".into(),
            ),
        };

        (status, Json(serde_json::json!({ "error": msg }))).into_response()
    }
}
