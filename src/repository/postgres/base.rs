use crate::repository::BaseRepo;
use sqlx::{Database, Error, Pool, Postgres, Transaction};
use std::sync::Arc;

pub struct BaseRepositoryImpl<DB: Database> {
    pool: Arc<Pool<DB>>,
}
impl BaseRepositoryImpl<Postgres> {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        BaseRepositoryImpl { pool }
    }
}

#[async_trait::async_trait]
impl BaseRepo<Postgres> for BaseRepositoryImpl<Postgres> {
    async fn transaction_begin(&self) -> Result<Transaction<'static, Postgres>, Error> {
        match self.pool.begin().await {
            Ok(transaction) => Ok(transaction),
            Err(error) => Err(Error::from(error)),
        }
    }

    async fn commit_transaction(
        &self,
        transaction: Transaction<'_, Postgres>,
    ) -> Result<(), Error> {
        match transaction.commit().await {
            Ok(_) => Ok(()),
            Err(error) => Err(Error::from(error)),
        }
    }

    async fn rollback_transaction(
        &self,
        transaction: Transaction<'_, Postgres>,
    ) -> Result<(), Error> {
        match transaction.rollback().await {
            Ok(_) => Ok(()),
            Err(error) => Err(Error::from(error)),
        }
    }
}
