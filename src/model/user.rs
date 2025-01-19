use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub birthdate: Option<NaiveDateTime>,
    pub gender: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: NaiveDateTime,
}
