pub mod credential;
pub mod user;

use crate::{debug_info, model};
use async_trait::async_trait;
use sqlx::pool::PoolOptions;
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

    pub async fn create_pool(
        app_config: &crate::config::config::Config,
    ) -> Result<Pool<Postgres>, Error> {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}?application_name=rustrest",
            app_config.database().db_username(),
            app_config.database().db_password(),
            app_config.database().db_host(),
            app_config.database().db_port(),
            app_config.database().db_name(),
        );

        // create pool koneksi ke PostgreSQL
        let pool = PoolOptions::new()
            .max_connections(10)
            .min_connections(2)
            .connect(database_url.as_str())
            .await?;

        //checking
        let row: (i64,) = sqlx::query_as(
            "SELECT count(*) FROM pg_stat_activity WHERE application_name='rustrest'",
        )
        .fetch_one(&pool)
        .await?;
        debug_info!("Checking Result of SELECT count(*) FROM pg_stat_activity WHERE application_name='rustrest' : {:?}", row);

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
