mod schema;
pub mod v1;

use crate::config::config::Config;
use crate::http_handler::v1::credential::handler as credential_handler;
use crate::service::credential::CredentialServiceImpl;
use actix_web::web;
use std::sync::Arc;

pub fn init(
    actix_cfg: &mut web::ServiceConfig,
    app_cfg: Arc<Config>,
    credential_service: Arc<CredentialServiceImpl>,
) {
    let handler = credential_handler::CredentialHandler::new(credential_service, app_cfg);
    handler.configuration_v1(actix_cfg)
}
