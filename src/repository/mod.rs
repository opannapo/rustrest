pub mod credential;
pub mod user;

use async_trait::async_trait;
use crate::model;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use sqlx::Error;
use tokio_postgres::NoTls;

pub struct PostgresPool {
    pub pool: Pool,
}
impl PostgresPool {
    pub fn new(app_config: &crate::config::config::Config) -> PostgresPool {
        PostgresPool {
            pool: Self::create_pool(app_config),
        }
    }

    pub fn create_pool(app_config: &crate::config::config::Config) -> Pool {
        //DATABASE_URL=postgres://username:password@localhost/dbname
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            app_config.database().db_username(),
            app_config.database().db_password(),
            app_config.database().db_host(),
            app_config.database().db_port(),
            app_config.database().db_name(),
        );

        let mut cfg = Config::new();
        cfg.dbname = Some(database_url);
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });
        cfg.create_pool(None, NoTls).expect("Failed to create pool")
    }
}
#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn create(&self, model: model::user::User) -> Result<(), Error>;
    async fn get_by_id(&self, id: i64) -> Result<model::user::User, Error>;
}
#[async_trait]
pub trait CredentialRepo: Send + Sync {
    async fn create(&self, model: model::credential::Credential) -> Result<(), Error>;
    async fn get_by_username(
        &self,
        username: String,
    ) -> Result<model::credential::Credential, Error>;
}
