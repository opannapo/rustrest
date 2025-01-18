use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Credential {
    pub username: String,
    pub password_hash: String,
    pub user_id: String,
    pub status: Option<i32>, // `status` bisa bernilai NULL, jadi menggunakan Option
}
