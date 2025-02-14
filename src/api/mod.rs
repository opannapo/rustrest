mod schema;
pub mod v1;

use crate::api::v1::auth::handler as cred_handler;
use crate::api::v1::user::handler as usr_handler;
use crate::config::config::Config;
use crate::service::auth::service::AuthServiceImpl;
use crate::service::user::UserServiceImpl;
use actix_web::web;
use std::sync::Arc;

pub fn init(
    actix_cfg: &mut web::ServiceConfig,
    app_cfg: Arc<Config>,
    user_service: Arc<UserServiceImpl>,
    auth_service: Arc<AuthServiceImpl>,
) {
    cred_handler::CredentialHandler::new(auth_service.clone(), app_cfg.clone())
        .configuration_v1(actix_cfg);
    usr_handler::UserHandler::new(user_service, app_cfg.clone()).configuration_v1(actix_cfg);
}
