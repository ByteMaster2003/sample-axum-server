use crate::{error::AppError, models::user::User};
use mongodb::{Database, bson};

pub async fn create_user(db: &Database, user: User) -> Result<(), AppError> {
    let collection = db.collection::<User>("users");
    collection
        .insert_one(user)
        .await
        .map_err(AppError::DatabaseError)?;
    Ok(())
}

pub async fn find_user_by_email(
    db: &Database,
    email: &str,
) -> Result<Option<User>, AppError> {
    let collection = db.collection::<User>("users");
    collection
        .find_one(bson::doc! { "email": email })
        .await
        .map_err(AppError::DatabaseError)
}
