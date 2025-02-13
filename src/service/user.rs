use crate::api::v1::user::schema::UserResponse;
use crate::repository::UserRepo;
use crate::service::UserService;
use sqlx::Postgres;
use std::sync::Arc;

pub struct UserServiceImpl {
    user_repo: Arc<dyn UserRepo<Postgres>>,
}
impl UserServiceImpl {
    pub fn new(user_repo: Arc<dyn UserRepo<Postgres>>) -> Self {
        UserServiceImpl { user_repo }
    }
}

impl UserService for UserServiceImpl {
    fn get_by_username(&self, username: &str) -> UserResponse {
        let random_uuid = uuid::Uuid::new_v4();
        UserResponse {
            username: username.to_string(),
            user_id: random_uuid.to_string(),
            password: String::from("ssss"),
        }
    }
}
