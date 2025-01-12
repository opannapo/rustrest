use crate::api::credential::repo::CredentialRepo;

pub trait CredentialService {
    fn auth(&self, username: &str, password: &str) -> &str;
}

pub struct CredentialServiceImpl<CR: CredentialRepo>
where
    CR: CredentialRepo,
{
    repo: CR,
}

impl<CR> CredentialServiceImpl<CR>
where
    CR: CredentialRepo,
{
    pub fn new(repo: CR) -> Self {
        CredentialServiceImpl { repo }
    }
}

impl<CR> CredentialService for CredentialServiceImpl<CR>
where
    CR: CredentialRepo,
{
    fn auth(&self, username: &str, password: &str) -> &str {
        // println!("adfadf");
        "adfadf"
    }
}
