use crate::api::v1::user::schema::UserResponse;
use async_trait::async_trait;
use std::error::Error;

pub mod auth;
pub mod user;

pub trait UserService {
    fn get_by_username(&self, username: &str) -> UserResponse;
}

#[async_trait]
pub trait AuthService {
    async fn signup(
        &self,
        request: auth::schema::SignupRequest,
    ) -> Result<auth::schema::SignupResponse, Box<dyn Error>>;
}
