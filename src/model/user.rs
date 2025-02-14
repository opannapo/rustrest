use chrono::{NaiveDate, NaiveDateTime};
use serde_json::json;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub gender: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: NaiveDateTime,
}
impl User {
    pub fn to_typesense_json(&self) -> serde_json::Value {
        json!({
            "id": self.id.to_string(),
            "name": self.name,
            "birthdate": self.birthdate,
            "gender": self.gender,
            "location": [self.latitude, self.longitude],  // ✅ GeoPoint
            "created_at": self.created_at.timestamp()
        })
    }
}
