use crate::{
    dto::{auth::Claims, user::UserInfoResponse},
    error::AppError,
    repository::user_repo,
};

pub async fn get_user_info(
    db: &mongodb::Database,
    claims: Claims,
) -> Result<UserInfoResponse, AppError> {
    // 1. Check if user exists
    let user = match user_repo::find_user_by_email(db, &claims.email).await? {
        Some(u) => u,
        None => return Err(AppError::NotFound("User not found".into())),
    };

    let user_id = match user.id {
        Some(id) => id.to_string(),
        None => "Unknown".to_string(),
    };

    Ok(UserInfoResponse {
        id: user_id,
        email: user.email,
    })
}
