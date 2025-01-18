pub mod handler;
pub mod repo;
pub mod schema;
pub mod service;

use crate::api::v1::credential::handler as credential_handler;
use crate::api::v1::credential::repo as credential_repo;
use crate::api::v1::credential::service as credential_service;
use actix_web::web;
use std::sync::Arc;

pub fn init(cfg: &mut web::ServiceConfig) {
    let repo = credential_repo::CredentialRepoImpl::new();
    let service = credential_service::CredentialServiceImpl::new(repo);
    let handler = credential_handler::CredentialHandler::new(Arc::new(service));
    handler.configuration_v1(cfg)
}
