use crate::repository::CredentialRepo;
use deadpool_postgres::Pool;
use std::sync::Arc;

pub struct CredentialRepoImpl {
    pool: Arc<Pool>,
}
impl CredentialRepoImpl {
    pub fn new(pool: Arc<Pool>) -> Self {
        return CredentialRepoImpl { pool };
    }
}

impl CredentialRepo for CredentialRepoImpl {
    fn create(&self) {
        todo!()
    }

    fn get_by_username(&self, username: String) {
        todo!()
    }
}
