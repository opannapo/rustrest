use crate::common::internal_error::InternalError;
use crate::api::v1::auth::schema::AuthResponse;
use crate::repository::{BaseRepo, CredentialRepo, UserRepo};
use crate::service::CredentialService;
use crate::{debug_error, debug_info, model};
use async_trait::async_trait;
use bcrypt::hash;
use log::error;
use serde::__private::de::IdentifierDeserializer;
use sqlx::{Postgres, Transaction};
use std::error::Error;
use std::sync::Arc;

pub struct CredentialServiceImpl {
    base_repo: Arc<dyn BaseRepo<Postgres>>,
    credential_repo: Arc<dyn CredentialRepo<Postgres>>,
    user_repo: Arc<dyn UserRepo<Postgres>>,
}
impl CredentialServiceImpl {
    pub fn new(
        base_repo: Arc<dyn BaseRepo<Postgres>>,
        credential_repo: Arc<dyn CredentialRepo<Postgres>>,
        user_repo: Arc<dyn UserRepo<Postgres>>,
    ) -> Self {
        CredentialServiceImpl {
            base_repo,
            credential_repo,
            user_repo,
        }
    }
}

#[async_trait]
impl CredentialService for CredentialServiceImpl {
    async fn create(&self, username: &str, password: &str) -> Result<AuthResponse, Box<dyn Error>> {
        debug_info!("Creating credential");

        let random_uuid = uuid::Uuid::new_v4();
        // let new_username = format!("{}@{}", username, random_uuid);
        let pwd_hash;
        match hash(&password, 4) {
            Ok(val) => pwd_hash = val,
            Err(err) => return Err(Box::new(err)),
        }

        let mut tx: Transaction<Postgres>;
        let tx_result = self.base_repo.transaction_begin().await;
        match tx_result {
            Ok(val) => tx = val,
            Err(err) => return Err(Box::new(err)),
        }

        let now = chrono::Utc::now().naive_utc();

        match self
            .user_repo
            .create(
                model::user::User {
                    created_at: now,
                    birthdate: Some(now),
                    gender: None,
                    id: random_uuid,
                    latitude: 0.0,
                    longitude: 0.0,
                    name: Option::from(username.to_string()),
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
                    username: username.to_string(),
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

        Ok(AuthResponse {
            username: username.to_string(),
            password: password.to_string(),
            user_id: random_uuid.to_string(),
        })
    }
}
