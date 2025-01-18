use crate::http_handler::v1::auth::schema::AuthResponse;
use crate::repository::{CredentialRepo, UserRepo};
use crate::service::CredentialService;
use crate::{debug_info, model};
use async_trait::async_trait;
use dotenv::Error;
use serde::de::Unexpected::Option;
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

#[async_trait]
impl CredentialService for CredentialServiceImpl {
    async fn create(&self, username: &str, password: &str) -> Result<AuthResponse, Error> {
        debug_info!("Creating credential");

        let random_uuid = uuid::Uuid::new_v4();
        let password_hash = password.to_string();

        let _ = self
            .credential_repo
            .create(model::credential::Credential {
                user_id: random_uuid.to_string(),
                username: username.to_string(),
                status: Some(1),
                password_hash,
            })
            .await
            .unwrap();

        Ok(AuthResponse {
            username: username.to_string(),
            password: password.to_string(),
            user_id: random_uuid.to_string(),
        })
    }
}
