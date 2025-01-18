use crate::http_handler::v1::auth::schema::AuthResponse;
use crate::http_handler::v1::user::schema::UserResponse;
use async_trait::async_trait;
use std::error::Error;

pub mod credential;
pub mod user;

pub trait UserService {
    fn get_by_username(&self, username: &str) -> UserResponse;
}

#[async_trait]
pub trait CredentialService {
    async fn create(&self, username: &str, password: &str) -> Result<AuthResponse, Box<dyn Error>>;
}
