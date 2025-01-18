use crate::model::credential::Credential;
use crate::repository::CredentialRepo;
use deadpool_postgres::Pool;
use sqlx::Error;
use std::sync::Arc;
use async_trait::async_trait;

pub struct CredentialRepoImpl {
    pool: Arc<Pool>,
}
impl CredentialRepoImpl {
    pub fn new(pool: Arc<Pool>) -> Self {
        return CredentialRepoImpl { pool };
    }
}
#[async_trait]
impl CredentialRepo for CredentialRepoImpl {
    async fn create(&self, model: Credential) -> Result<(), Error> {
        todo!()
    }

    async fn get_by_username(&self, username: String) -> Result<Credential, Error> {
        todo!()
    }
}
