use crate::debug_info;
use crate::model::credential::Credential;
use crate::repository::CredentialRepo;
use async_trait::async_trait;
use sqlx::{Database, Error, Pool, Postgres, Transaction};
use std::sync::Arc;

pub struct CredentialRepoImpl<DB: Database> {
    pool: Arc<Pool<DB>>,
}
impl CredentialRepoImpl<Postgres> {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        return CredentialRepoImpl { pool };
    }
}
#[async_trait]
impl CredentialRepo<Postgres> for CredentialRepoImpl<Postgres> {
    async fn create(
        &self,
        model: Credential,
        tx: Option<&mut Transaction<'_, Postgres>>,
    ) -> Result<(), Error> {
        debug_info!("");

        sqlx::query(
            "INSERT INTO credential (username, password_hash, user_id, status) VALUES ($1, $2, $3, $4)",
        )
        .bind(model.username)
        .bind(model.password_hash)
        .bind(model.user_id)
        .bind(model.status)
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn get_by_username(&self, username: String) -> Result<Credential, Error> {
        todo!()
    }
}
