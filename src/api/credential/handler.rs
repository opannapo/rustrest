use crate::api::credential::schema;
use crate::api::credential::service::CredentialService;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::Json;
use actix_web::{post, web, HttpResponse, Responder};
use std::sync::Arc;

pub struct CredentialHandler {
    pub credential_service_impl: Arc<dyn CredentialService>,
}

impl CredentialHandler {
    pub fn new(credential_service_impl: Arc<dyn CredentialService>) -> Self {
        CredentialHandler {
            credential_service_impl,
        }
    }

    pub fn configuration_v1(&self, cfg: &mut web::ServiceConfig) {
        let scope = web::scope("/v1/credential")
            .app_data(web::Data::new(self.credential_service_impl.clone()))
            .service(credential_auth);
        cfg.service(scope);
    }
}

#[post("/auth")]
async fn credential_auth(
    req: Json<schema::AuthRequest>,
    credential_service: web::Data<Arc<dyn CredentialService>>,
) -> impl Responder {
    let result = credential_service.auth(req.username.as_str(), req.password.as_str());
    let response = schema::AuthResponse {
        user: format!(
            "username is {} password is {}, service return is {}",
            req.username, req.password, result
        ),
    };
    HttpResponse::Ok().json(response)
}
