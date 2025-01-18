use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub user_id: String,
    pub username: String,
    pub password: String,
}
