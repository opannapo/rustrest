use crate::http_handler::v1::credential::schema::AuthResponse;

pub mod credential;

pub trait CredentialService {
    fn auth(&self, username: &str, password: &str) -> AuthResponse;
}
