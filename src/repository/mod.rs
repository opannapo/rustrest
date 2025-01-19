pub mod postgres;

use crate::model;
use async_trait::async_trait;
use sqlx::{Database, Error, Postgres, Transaction};

#[async_trait]
pub trait BaseRepo<DB: Database>: Send + Sync {
    async fn transaction_begin(&self) -> Result<Transaction<'static, DB>, Error>;
    async fn commit_transaction(&self, transaction: Transaction<'_, DB>) -> Result<(), Error>;
    async fn rollback_transaction(&self, transaction: Transaction<'_, DB>) -> Result<(), Error>;
}
#[async_trait]
pub trait UserRepo<DB: Database>: Send + Sync {
    async fn create(
        &self,
        model: model::user::User,
        tx: Option<&mut Transaction<'_, Postgres>>,
    ) -> Result<(), Error>;
    async fn get_by_id(&self, id: i64) -> Result<model::user::User, Error>;
}
#[async_trait]
pub trait CredentialRepo<DB: Database>: Send + Sync {
    async fn create(
        &self,
        model: model::credential::Credential,
        tx: Option<&mut Transaction<'_, Postgres>>,
    ) -> Result<(), Error>;
    async fn get_by_username(
        &self,
        username: String,
    ) -> Result<model::credential::Credential, Error>;
}
