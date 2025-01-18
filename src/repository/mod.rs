pub mod credential;
pub mod user;

pub trait UserRepo {
    fn create(&self);
    fn get_by_id(&self, id: i64);
}

pub trait CredentialRepo {
    fn create(&self);
    fn get_by_username(&self, username: String);
}
