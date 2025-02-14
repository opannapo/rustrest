use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupRequest {
    pub credential_username: String,
    pub credential_password: String,
    pub user_name: String,
    pub user_birthdate: String,
    pub user_gender: String,
    pub user_latitude: f64,
    pub user_longitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupResponse {
    pub user_id: String,
}
