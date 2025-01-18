use crate::http_handler::v1::credential::schema::AuthResponse;
use crate::repository::{CredentialRepo, UserRepo};
use crate::service::CredentialService;
use std::sync::Arc;

pub struct CredentialServiceImpl {
    credential_repo: Arc<dyn CredentialRepo>,
    user_repo: Arc<dyn UserRepo>,
}
impl CredentialServiceImpl {
    pub fn new(credential_repo: Arc<dyn CredentialRepo>, user_repo: Arc<dyn UserRepo>) -> Self {
        CredentialServiceImpl {
            credential_repo,
            user_repo,
        }
    }
}

impl CredentialService for CredentialServiceImpl {
    fn auth(&self, username: &str, password: &str) -> AuthResponse {
        let random_uuid = uuid::Uuid::new_v4();
        AuthResponse {
            username: username.to_string(),
            password: password.to_string(),
            user_id: random_uuid.to_string(),
        }
    }
}
