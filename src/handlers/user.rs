use crate::{
    data::SharedState, dto::auth::Claims, error::AppError, response::ApiResponse,
    services::user_service,
};
use axum::{Extension, extract::State, response::IntoResponse};

pub async fn get_user_info(
    Extension(claims): Extension<Claims>,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, AppError> {
    let data = user_service::get_user_info(&state.db, claims).await?;

    Ok(ApiResponse::success(
        data,
        "User profile fetched successfully",
    ))
}
