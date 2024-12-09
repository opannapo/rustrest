pub trait CredentialRepo {
    fn get_credential(&self);
}

pub struct CredentialRepoImpl;
impl CredentialRepoImpl {
    pub fn new() -> Self {
        CredentialRepoImpl {}
    }
}

impl CredentialRepo for CredentialRepoImpl {
    fn get_credential(&self) {
        todo!()
    }
}
