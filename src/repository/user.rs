use crate::repository::UserRepo;
use deadpool_postgres::Pool;
use std::sync::Arc;

pub struct UserRepoImpl {
    pool: Arc<Pool>,
}
impl UserRepoImpl {
    pub fn new(pool: Arc<Pool>) -> Self {
        return UserRepoImpl { pool };
    }
}

impl UserRepo for UserRepoImpl {
    fn create(&self) {
        todo!()
    }

    fn get_by_id(&self, id: i64) {
        todo!()
    }
}
