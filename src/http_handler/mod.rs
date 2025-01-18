mod schema;
pub mod v1;

use crate::config::config::Config;
use crate::http_handler::v1::auth::handler as cred_handler;
use crate::http_handler::v1::user::handler as usr_handler;
use crate::service::credential::CredentialServiceImpl;
use crate::service::user::UserServiceImpl;
use actix_web::web;
use std::sync::Arc;

pub fn init(
    actix_cfg: &mut web::ServiceConfig,
    app_cfg: Arc<Config>,
    credential_service: Arc<CredentialServiceImpl>,
    user_service: Arc<UserServiceImpl>,
) {
    cred_handler::CredentialHandler::new(credential_service, app_cfg.clone()).configuration_v1(actix_cfg);
    usr_handler::UserHandler::new(user_service, app_cfg.clone()).configuration_v1(actix_cfg);
}
