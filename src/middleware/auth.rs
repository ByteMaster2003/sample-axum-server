use crate::{error::AppError, utils};
use axum::{extract::Request, middleware::Next, response::Response};

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = match auth_header.strip_prefix("Bearer ") {
        Some(t) => t,
        None => return Err(AppError::Unauthorized),
    };

    let token_data = utils::jwt::decode_token(token)?;
    let claims = token_data.claims;

    // Inject the user_id into the request extensions for the handler to use
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
