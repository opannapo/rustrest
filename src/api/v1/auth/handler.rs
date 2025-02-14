use crate::api::schema::request::RequestContext;
use crate::api::schema::response;
use crate::config::config::Config;
use crate::service::auth::schema as auth_service_schema;
use crate::service::AuthService;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest, Responder};
use std::sync::Arc;

pub struct CredentialHandler {
    pub auth_service_impl: Arc<dyn AuthService>,
    pub app_config: Arc<Config>,
}

impl CredentialHandler {
    pub fn new(auth_service_impl: Arc<dyn AuthService>, app_config: Arc<Config>) -> Self {
        CredentialHandler {
            auth_service_impl,
            app_config,
        }
    }

    pub fn configuration_v1(&self, cfg: &mut web::ServiceConfig) {
        let scope = web::scope("/v1/auth")
            .app_data(web::Data::new(self.auth_service_impl.clone()))
            .service(credential_auth);
        cfg.service(scope);
    }
}

#[post("/signup")]
async fn credential_auth(
    req: HttpRequest,
    req_payload: Json<auth_service_schema::SignupRequest>,
    auth_service: web::Data<Arc<dyn AuthService>>,
) -> impl Responder {
    let ctx = RequestContext::new(req);
    let result = auth_service
        .signup(auth_service_schema::SignupRequest {
            credential_username: req_payload.credential_username.clone(),
            credential_password: req_payload.credential_password.clone(),
            user_name: req_payload.user_name.clone(),
            user_birthdate: req_payload.user_birthdate.clone(),
            user_gender: req_payload.user_gender.clone(),
            user_latitude: req_payload.user_latitude.clone(),
            user_longitude: req_payload.user_longitude.clone(),
        })
        .await;
    match result {
        Ok(ok) => response::ok(ctx, ok),
        Err(err) => response::err(ctx, err),
    }
}
