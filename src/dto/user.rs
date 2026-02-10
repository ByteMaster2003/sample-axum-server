#[derive(serde::Serialize)]
pub struct UserInfoResponse {
    pub id: String,
    pub email: String,
}
