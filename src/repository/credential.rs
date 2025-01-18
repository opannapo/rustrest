use crate::repository::CredentialRepo;

pub struct CredentialRepoImpl;
impl CredentialRepoImpl {
    pub fn new() -> Self {
        return CredentialRepoImpl {};
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
