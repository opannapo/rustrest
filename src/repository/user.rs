use crate::model::user::User;
use crate::repository::UserRepo;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use sqlx::Error;
use std::sync::Arc;

pub struct UserRepoImpl {
    pool: Arc<Pool>,
}
impl UserRepoImpl {
    pub fn new(pool: Arc<Pool>) -> Self {
        return UserRepoImpl { pool };
    }
}

#[async_trait]
impl UserRepo for UserRepoImpl {
    async fn create(&self, model: User) -> Result<(), Error> {
        todo!()
    }

    async fn get_by_id(&self, id: i64) -> Result<User, Error> {
        todo!()
    }
}
