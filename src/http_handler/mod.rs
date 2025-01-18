mod schema;
pub mod v1;

use crate::http_handler::v1::credential::handler as credential_handler;
use crate::repository::{credential, user};
use crate::service::credential as credential_service;
use actix_web::web;
use std::sync::Arc;

pub fn init(cfg: &mut web::ServiceConfig) {
    let cred_repo = credential::CredentialRepoImpl::new();
    let user_repo = user::UserRepoImpl::new();

    let service =
        credential_service::CredentialServiceImpl::new(Arc::new(cred_repo), Arc::new(user_repo));

    let handler = credential_handler::CredentialHandler::new(Arc::new(service));
    handler.configuration_v1(cfg)
}
