use crate::config::config::Config;
use crate::http_handler::schema::request::RequestContext;
use crate::http_handler::schema::response;
use crate::http_handler::v1::credential::schema;
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
        let scope = web::scope("/v1/credential")
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
    let result =
        credential_service.auth(req_payload.username.as_str(), req_payload.password.as_str());
    response::ok(ctx, result)
}
