use actix_web::{App, HttpServer};
use rustrest::config::config::Config;
use rustrest::repository::{credential, user};
use rustrest::service::credential::CredentialServiceImpl;
use rustrest::util::log as custom_log;
use rustrest::{debug_info, http_handler, repository};
use std::sync::Arc;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    custom_log::init();
    debug_info!("main api");

    let app_cfg = Arc::new(Config::new());
    debug_info!("main api {:?}", &app_cfg);

    let pg_pool = repository::PostgresPool::new(&app_cfg);
    let pool = Arc::new(pg_pool.pool);
    let cred_repo = Arc::new(credential::CredentialRepoImpl::new(Arc::clone(&pool)));
    let user_repo = Arc::new(user::UserRepoImpl::new(Arc::clone(&pool)));
    let service = Arc::new(CredentialServiceImpl::new(cred_repo, user_repo));

    HttpServer::new(move || {
        let app_cfg = app_cfg.clone();
        let service = service.clone();
        App::new().configure(move |cfg| http_handler::init(cfg, app_cfg, service))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1) //bikin auto aja ngikutin cpu thread -> comment untuk pakei default total cpu core
    .run()
    .await
}
