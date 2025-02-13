use crate::api::schema::request::RequestContext;
use crate::api::schema::response;
use crate::api::v1::auth::schema;
use crate::config::config::Config;
use crate::service::CredentialService;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest, Responder};
use std::sync::Arc;

pub struct CredentialHandler {
    pub credential_service_impl: Arc<dyn CredentialService>,
    pub app_config: Arc<Config>,
}

impl CredentialHandler {
    pub fn new(
        credential_service_impl: Arc<dyn CredentialService>,
        app_config: Arc<Config>,
    ) -> Self {
        CredentialHandler {
            credential_service_impl,
            app_config,
        }
    }

    pub fn configuration_v1(&self, cfg: &mut web::ServiceConfig) {
        let scope = web::scope("/v1/auth")
            .app_data(web::Data::new(self.credential_service_impl.clone()))
            .service(credential_auth);
        cfg.service(scope);
    }
}

#[post("/signup")]
async fn credential_auth(
    req: HttpRequest,
    req_payload: Json<schema::AuthRequest>,
    credential_service: web::Data<Arc<dyn CredentialService>>,
) -> impl Responder {
    let ctx = RequestContext::new(req);
    let result = credential_service
        .create(req_payload.username.as_str(), req_payload.password.as_str())
        .await;
    match result {
        Ok(user) => response::ok(ctx, user),
        Err(err) => response::err(ctx, err),
    }
}
