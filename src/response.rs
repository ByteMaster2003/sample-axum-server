use axum::{Json, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct EmptyData;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    // Helper for successful responses with data
    pub fn success(data: T, message: &str) -> (StatusCode, Json<ApiResponse<T>>) {
        (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: message.to_string(),
                data: Some(data),
            }),
        )
    }

    // Helper for successful actions with no data (like Delete or Logout)
    pub fn msg(message: &str) -> (StatusCode, Json<ApiResponse<()>>) {
        (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                message: message.to_string(),
                data: None,
            }),
        )
    }
}

#[derive(Serialize)]
pub struct PaginatedData<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
}
