use serde::{Deserialize, Serialize};
use validator::Validate;

// Register Schema
#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {}

// Login Schema
#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub id: String,
    pub email: String,
    pub token: String,
}

// Claims Schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub email: String,

    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub aud: String,
    pub nbf: usize,
}
