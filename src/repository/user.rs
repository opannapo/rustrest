use crate::model::user::User;
use crate::repository::UserRepo;
use async_trait::async_trait;
// use deadpool_postgres::Pool;
use sqlx::{Database, Error, Pool, Postgres, Transaction};
use std::sync::Arc;

pub struct UserRepoImpl<DB: Database> {
    pool: Arc<Pool<DB>>,
}
impl UserRepoImpl<Postgres> {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        return UserRepoImpl { pool };
    }
}

#[async_trait]
impl UserRepo<Postgres> for UserRepoImpl<Postgres> {
    async fn create(&self, model: User, tx: Transaction<'_, Postgres>) -> Result<(), Error> {
        todo!()
    }

    async fn get_by_id(&self, id: i64) -> Result<User, Error> {
        todo!()
    }
}
