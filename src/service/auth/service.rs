use crate::common::internal_error::InternalError;
use crate::repository::{BaseRepo, CredentialRepo, UserRepo};
use crate::service::auth::schema::{SignupRequest, SignupResponse};
use crate::service::user::UserServiceImpl;
use crate::service::AuthService;
use crate::util::location::generate_random_lat_lon;
use crate::{debug_error, debug_info, model};
use async_trait::async_trait;
use bcrypt::hash;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{Postgres, Transaction};
use std::error::Error;
use std::sync::Arc;

pub struct AuthServiceImpl {
    base_repo: Arc<dyn BaseRepo<Postgres>>,
    credential_repo: Arc<dyn CredentialRepo<Postgres>>,
    user_repo: Arc<dyn UserRepo<Postgres>>,
}
impl AuthServiceImpl {
    pub fn new(
        base_repo: Arc<dyn BaseRepo<Postgres>>,
        credential_repo: Arc<dyn CredentialRepo<Postgres>>,
        user_repo: Arc<dyn UserRepo<Postgres>>,
    ) -> Self {
        AuthServiceImpl {
            base_repo,
            user_repo,
            credential_repo,
        }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn signup(&self, request: SignupRequest) -> Result<SignupResponse, Box<dyn Error>> {
        debug_info!("Creating credential");

        let random_uuid = uuid::Uuid::new_v4();
        let pwd_hash;
        match hash(&request.credential_password, 4) {
            Ok(val) => pwd_hash = val,
            Err(err) => return Err(Box::new(err)),
        }

        let mut tx: Transaction<Postgres>;
        let tx_result = self.base_repo.transaction_begin().await;
        match tx_result {
            Ok(val) => tx = val,
            Err(err) => return Err(Box::new(err)),
        }

        let format = "%Y-%m-%d";
        let now = chrono::Utc::now().naive_utc();
        let mut user_birthdate: Option<NaiveDate> = None;
        match NaiveDate::parse_from_str(&request.user_birthdate, format) {
            Ok(date) => user_birthdate = Some(date),
            Err(e) => println!("Error parsing date: {}", e),
        }

        match self
            .user_repo
            .create(
                model::user::User {
                    created_at: now,
                    birthdate: user_birthdate,
                    gender: Some(request.user_gender),
                    id: random_uuid,
                    latitude: request.user_latitude,
                    longitude: request.user_longitude,
                    name: Some(request.user_name),
                },
                Some(&mut tx),
            )
            .await
        {
            Ok(_) => {}
            Err(err) => {
                debug_error!("{}", err);
                tx.rollback().await?;

                let err = InternalError::db_exec(format!("create user {}", err).as_str());
                return Err(Box::new(err));
            }
        }

        match self
            .credential_repo
            .create(
                model::credential::Credential {
                    user_id: random_uuid,
                    username: request.credential_username,
                    status: Some(1),
                    password_hash: pwd_hash.to_string(),
                    created_at: chrono::Utc::now(),
                },
                Some(&mut tx),
            )
            .await
        {
            Ok(()) => {}
            Err(err) => {
                debug_error!("{}", err);
                tx.rollback().await?;

                let err = InternalError::db_exec(format!("create credential {}", err).as_str());
                return Err(Box::new(err));
            }
        }

        tx.commit().await?;

        Ok(SignupResponse {
            user_id: random_uuid.to_string(),
        })
    }
}
