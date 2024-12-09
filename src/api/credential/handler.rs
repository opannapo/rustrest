use crate::api::credential::schema;
use crate::api::credential::service::CredentialService;
use actix_web::web::Json;
use actix_web::{post, web, HttpResponse, Responder};

pub struct CredentialHandler<CS>
where
    CS: CredentialService,
{
    pub credential_service_impl: CS,
}

impl<CS> CredentialHandler<CS>
where
    CS: CredentialService,
{
    pub fn new(credential_service_impl: CS) -> Self {
        CredentialHandler {
            credential_service_impl,
        }
    }

    pub fn configuration_v1(&self, cfg: &mut web::ServiceConfig) {
        let scope = web::scope("/v1/credential").service(credential_auth);
        cfg.service(scope);
    }
}

#[post("/auth")]
async fn credential_auth(req: Json<schema::AuthRequest>) -> impl Responder {
    let response = schema::AuthResponse {
        user: format!("username is {} password is {}", req.username, req.password),
    };
    HttpResponse::Ok().json(response)
}
