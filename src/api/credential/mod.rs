pub mod handler;
pub mod repo;
mod schema;
pub mod service;

use crate::api::credential::handler as credential_handler;
use crate::api::credential::repo as credential_repo;
use crate::api::credential::service as credential_service;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    let repo = credential_repo::CredentialRepoImpl::new();
    let service = credential_service::CredentialServiceImpl::new(repo);
    let handler = credential_handler::CredentialHandler::new(service);
    handler.configuration_v1(cfg)
}
