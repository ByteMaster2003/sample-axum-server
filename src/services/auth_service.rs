use crate::{
    dto::auth::{LoginRequest, LoginResponse, RegisterRequest},
    error::AppError,
    models::user::User,
    repository::user_repo,
    utils,
};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub async fn register(db: &mongodb::Database, req: RegisterRequest) -> Result<(), AppError> {
    // 1. Check if user exists
    if user_repo::find_user_by_email(db, &req.email)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("User already exists".into()));
    }

    // 2. Hash Password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|_| AppError::Internal)?
        .to_string();

    // 3. Save
    let new_user = User {
        id: None,
        email: req.email,
        password_hash,
    };
    user_repo::create_user(db, new_user).await
}

pub async fn login(db: &mongodb::Database, req: LoginRequest) -> Result<LoginResponse, AppError> {
    // 1. Check if user exists
    let user = match user_repo::find_user_by_email(db, &req.email).await? {
        Some(u) => u,
        None => return Err(AppError::NotFound("User not found".into())),
    };

    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| AppError::Internal)?;
    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized)?;

    let user_id = match user.id {
        Some(id) => id.to_string(),
        None => "Unknown".to_string(),
    };
    let token = utils::jwt::encode_token(user_id.clone(), user.email.clone())?;

    Ok(LoginResponse {
        id: user_id,
        email: user.email,
        token,
    })
}
