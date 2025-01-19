use crate::model::user::User;
use crate::repository::UserRepo;
use crate::{debug_error, debug_info};
use async_trait::async_trait;
use chrono::{NaiveDateTime, NaiveTime};
use sqlx::{Database, Error, Pool, Postgres, Transaction};
use std::ptr::null;
use std::sync::Arc;

pub struct UserRepoImpl<DB: Database> {
    pool: Arc<Pool<DB>>,
}
impl UserRepoImpl<Postgres> {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        return UserRepoImpl { pool };
    }
}

#[async_trait]
impl UserRepo<Postgres> for UserRepoImpl<Postgres> {
    async fn create(
        &self,
        model: User,
        tx: Option<&mut Transaction<'_, Postgres>>,
    ) -> Result<(), Error> {
        debug_info!("");
        let q = sqlx::query(
            r#"INSERT INTO "user" (
                    id,
                    name,
                    birthdate,
                    gender,
                    latitude,
                    longitude
                ) VALUES ($1,$2,$3,$4,$5,$6)
                "#,
        )
        .bind(model.id)
        .bind(model.name)
        .bind(model.birthdate)
        .bind(model.gender)
        .bind(model.latitude)
        .bind(model.longitude);
        if let Some(tx) = tx {
            match q.execute(&mut **tx).await {
                Ok(pg_query_result) => {
                    debug_info!("{:?}", pg_query_result);
                    Ok(())
                }
                Err(e) => {
                    debug_error!("{:?}", e);
                    Err(e)
                }
            }
        } else {
            match q.execute(&*self.pool).await {
                Ok(pg_query_result) => {
                    debug_info!("{:?}", pg_query_result);
                    Ok(())
                }
                Err(e) => {
                    debug_error!("{:?}", e);
                    Err(e)
                }
            }
        }
    }

    async fn get_by_id(&self, id: i64) -> Result<User, Error> {
        todo!()
    }
}
