use crate::http_handler::v1::credential::schema::AuthResponse;
use crate::http_handler::v1::user::schema::UserResponse;

pub mod credential;
pub mod user;

pub trait UserService {
    fn get_by_username(&self, username: &str) -> UserResponse;
}

pub trait CredentialService {
    fn auth(&self, username: &str, password: &str) -> AuthResponse;
}
