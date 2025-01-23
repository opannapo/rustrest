use crate::model::credential::Credential;
use crate::repository::CredentialRepo;
use crate::{debug_error, debug_info};
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

        let q=sqlx::query(
            r#"
            INSERT INTO credential
                (username, password_hash, user_id, status) VALUES
                ($1, $2, $3, $4)
            "#,
        )
        .bind(model.username)
        .bind(model.password_hash)
        .bind(model.user_id)
        .bind(model.status);

        if let Some(tx) = tx {
            match q.execute(&mut **tx).await {
                Ok(pg_query_result) => {
                    debug_info!("{:?}", pg_query_result);
                    Ok(())
                }
                Err(e) => {
                    debug_error!("{:?}", e);
                    Err(e)
                }
            }
        } else {
            match q.execute(&*self.pool).await {
                Ok(pg_query_result) => {
                    debug_info!("{:?}", pg_query_result);
                    Ok(())
                }
                Err(e) => {
                    debug_error!("{:?}", e);
                    Err(e)
                }
            }
        }
    }

    async fn get_by_username(&self, username: String) -> Result<Credential, Error> {
        todo!()
    }
}
