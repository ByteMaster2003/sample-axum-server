use crate::{
    data::SharedState,
    dto::auth::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse},
    error::AppError,
    response::ApiResponse,
    services::auth_service,
};
use axum::{Json, extract::State, response::IntoResponse};
use validator::Validate;

pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Layer 3: Validation
    payload
        .validate()
        .map_err(|e| AppError::InvalidPayload(e.to_string()))?;

    // Layer 5: Service
    auth_service::register(&state.db, payload).await?;

    Ok(ApiResponse::<RegisterResponse>::msg(
        "Registration successful",
    ))
}

pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Layer 3: Validation
    payload
        .validate()
        .map_err(|e| AppError::InvalidPayload(e.to_string()))?;

    // Layer 5: Service
    let data = auth_service::login(&state.db, payload).await?;

    Ok(ApiResponse::<LoginResponse>::success(
        data,
        "Login Successful",
    ))
}
