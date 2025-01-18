use crate::model::credential::Credential;
use crate::repository::CredentialRepo;
// use deadpool_postgres::Pool;
use async_trait::async_trait;
use sqlx::{Error, Pool, Postgres};
use std::sync::Arc;

pub struct CredentialRepoImpl {
    pool: Arc<Pool<Postgres>>,
}
impl CredentialRepoImpl {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
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
