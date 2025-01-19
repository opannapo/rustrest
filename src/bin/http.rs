use actix_web::{App, HttpServer};
use rustrest::config::config::Config;
use rustrest::repository::postgres::postgres::PostgresPool;
use rustrest::repository::postgres::{base, credential, user};
use rustrest::service::credential::CredentialServiceImpl;
use rustrest::service::user::UserServiceImpl;
use rustrest::util::log as custom_log;
use rustrest::{debug_info, http_handler, repository};
use sqlx::{Error, Pool, Postgres};
use std::sync::Arc;
use uuid::Variant::Future;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    custom_log::init();

    let app_cfg = Arc::new(Config::new());
    debug_info!("main api {:?}", &app_cfg);

    let pool: Arc<Pool<Postgres>> = Arc::new(PostgresPool::new(&app_cfg).await.unwrap().pool);

    let base_repo = Arc::new(base::BaseRepositoryImpl::new(Arc::clone(&pool)));
    let cred_repo = Arc::new(credential::CredentialRepoImpl::new(Arc::clone(&pool)));
    let user_repo = Arc::new(user::UserRepoImpl::new(Arc::clone(&pool)));

    let cred_service = Arc::new(CredentialServiceImpl::new(
        base_repo.clone(),
        cred_repo.clone(),
        user_repo.clone(),
    ));
    let usr_service = Arc::new(UserServiceImpl::new(user_repo.clone()));

    HttpServer::new(move || {
        let app_cfg = app_cfg.clone();
        let cred_service = cred_service.clone();
        let usr_service = usr_service.clone();
        App::new().configure(move |cfg| http_handler::init(cfg, app_cfg, cred_service, usr_service))
    })
    .bind(("127.0.0.1", 8080))?
    // .workers(1) //bikin auto aja ngikutin cpu thread -> comment untuk pakei default total cpu core
    .shutdown_timeout(30)
    .run()
    .await
}
