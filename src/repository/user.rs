use crate::repository::UserRepo;

pub struct UserRepoImpl {}
impl UserRepoImpl {
    pub fn new() -> Self {
        return UserRepoImpl {};
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
