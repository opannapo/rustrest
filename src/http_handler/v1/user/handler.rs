use crate::config::config::Config;
use crate::http_handler::schema::request::RequestContext;
use crate::http_handler::schema::response;
use crate::service::UserService;
use actix_web::dev::HttpServiceFactory;
use actix_web::{get, web, HttpRequest, Responder};
use std::sync::Arc;

pub struct UserHandler {
    pub user_service_impl: Arc<dyn UserService>,
    pub app_config: Arc<Config>,
}

impl UserHandler {
    pub fn new(user_service_impl: Arc<dyn UserService>, app_config: Arc<Config>) -> Self {
        UserHandler {
            user_service_impl,
            app_config,
        }
    }

    pub fn configuration_v1(&self, cfg: &mut web::ServiceConfig) {
        let scope = web::scope("/v1/user")
            .app_data(web::Data::new(self.user_service_impl.clone()))
            .service(credential_auth);
        cfg.service(scope);
    }
}

#[get("/profile")]
async fn credential_auth(
    req: HttpRequest,
    user_service: web::Data<Arc<dyn UserService>>,
) -> impl Responder {
    let ctx = RequestContext::new(req);
    let result = user_service.get_by_username("opamnapo");
    response::ok(ctx, result)
}
