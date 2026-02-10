use crate::{data::APP_CONFIG, dto::auth::Claims, error::AppError};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn encode_token(id: String, email: String) -> Result<String, AppError> {
    let secret_key = match APP_CONFIG.get().clone() {
        Some(cfg) => cfg.jwt_secret.clone(),
        None => return Err(AppError::Internal),
    };

    // Get current time in seconds
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Define duration (e.g., 1 hour = 3600 seconds)
    let one_hour = 60 * 60;
    let data = Claims {
        sub: id.clone(),
        exp: (now + one_hour) as usize,
        iat: now as usize,
        iss: "null-talk-auth-service".to_string(),
        aud: "null-talk-web".to_string(),
        nbf: now as usize,

        id,
        email,
    };

    let encoding_key = EncodingKey::from_secret(secret_key.as_ref());
    let token = encode(&Header::default(), &data, &encoding_key).map_err(|_| AppError::Internal)?;

    Ok(token)
}

pub fn decode_token(token: &str) -> Result<TokenData<Claims>, AppError> {
    let secret_key = match APP_CONFIG.get().clone() {
        Some(cfg) => cfg.jwt_secret.clone(),
        None => return Err(AppError::Internal),
    };

    let decoding_key = DecodingKey::from_secret(secret_key.as_ref());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["null-talk-web"]);
    validation.set_issuer(&["null-talk-auth-service"]);

    let token_data = decode::<Claims>(token, &decoding_key, &validation).map_err(|e| {
        tracing::error!("Error: {}", e.to_string());
        return AppError::Unauthorized;
    })?;

    Ok(token_data)
}
