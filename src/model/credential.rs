use chrono::{DateTime, TimeZone, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Credential {
    pub username: String,
    pub password_hash: String,
    pub user_id: Uuid,
    pub status: Option<i32>, // `status` bisa bernilai NULL, jadi menggunakan Option
    pub created_at: DateTime<Utc>,
}
