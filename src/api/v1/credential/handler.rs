use crate::api::schema::request::RequestContext;
use crate::api::schema::response;
use crate::api::v1::credential::schema;
use crate::api::v1::credential::service::CredentialService;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest, Responder};
use chrono::{DateTime, Duration, Utc};
use std::os::linux::raw::stat;
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
    req: HttpRequest,
    req_payload: Json<schema::AuthRequest>,
    credential_service: web::Data<Arc<dyn CredentialService>>,
) -> impl Responder {
    let start_at: DateTime<Utc> = Utc::now();
    let result = credential_service.auth(req_payload.username.as_str(), req_payload.password.as_str());
    let random_uuid = uuid::Uuid::new_v4();
    let ctx = RequestContext {
        request: req,
        start_at: start_at,
    };

    response::ok(
        ctx,
        schema::AuthResponse {
            username: req_payload.username.to_string(),
            password: req_payload.password.to_string(),
            request_id: random_uuid.to_string(),
            service_result: result.to_string(),
        },
    )
}
