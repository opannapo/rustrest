pub mod credential;
pub mod user;

use crate::model;
use async_trait::async_trait;
use sqlx::{Error, Pool, Postgres};

pub struct PostgresPool {
    pub pool: Pool<Postgres>,
}
impl PostgresPool {
    pub async fn new(app_config: &crate::config::config::Config) -> Result<PostgresPool, Error> {
        Ok(PostgresPool {
            pool: Self::create_pool(app_config).await?,
        })
    }

    /*pub fn create_pool(app_config: &crate::config::config::Config) -> Pool {
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
    }*/

    pub async fn create_pool(
        app_config: &crate::config::config::Config,
    ) -> Result<Pool<Postgres>, Error> {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            app_config.database().db_username(),
            app_config.database().db_password(),
            app_config.database().db_host(),
            app_config.database().db_port(),
            app_config.database().db_name(),
        );

        // Membuat pool koneksi ke PostgreSQL
        let pool = Pool::connect(&database_url).await?;
        Ok(pool)
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
