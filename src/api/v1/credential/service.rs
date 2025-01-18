use crate::api::v1::credential::repo::CredentialRepo;
use crate::api::v1::credential::schema::AuthResponse;

pub trait CredentialService {
    fn auth(&self, username: &str, password: &str) -> AuthResponse;
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
    fn auth(&self, username: &str, password: &str) -> AuthResponse {
        let random_uuid = uuid::Uuid::new_v4();
        AuthResponse {
            username: username.to_string(),
            password: password.to_string(),
            user_id: random_uuid.to_string(),
        }
    }
}
