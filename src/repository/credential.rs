use crate::debug_info;
use crate::model::credential::Credential;
use crate::repository::CredentialRepo;
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
        debug_info!("");

        let result = sqlx::query(
            "INSERT INTO public.credential (username, password_hash, user_id, status) VALUES ($1, $2, $3, $4)",
        )
        .bind(model.username)
        .bind(model.password_hash)
        .bind(model.user_id)
        .bind(model.status)
        .execute(&*self.pool)
        .await?;

        Ok(())

        // Query ke pg_stat_activity
        /*let row: (i64,) = sqlx::query_as(
            "SELECT count(*) FROM pg_stat_activity WHERE application_name='rustrest'",
        )
            .fetch_one(&*self.pool) // Menggunakan dereferensi Arc
            .await?;

        debug_info!(
            "Checking Result of SELECT count(*) FROM pg_stat_activity WHERE application_name='rustrest' : {:?}",
            row
        );

        Ok(())*/
    }

    async fn get_by_username(&self, username: String) -> Result<Credential, Error> {
        todo!()
    }
}
