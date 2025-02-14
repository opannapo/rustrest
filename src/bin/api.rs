use actix_web::{App, HttpServer};
use rustrest::config::config::Config;
use rustrest::config::typesense::Typesense;
use rustrest::pkg::typesense::typesense;
use rustrest::repository::postgres::postgres::PostgresPool;
use rustrest::repository::postgres::{base, credential, user};
use rustrest::service::auth::service::AuthServiceImpl;
use rustrest::service::user::UserServiceImpl;
use rustrest::util::log as custom_log;
use rustrest::{api, debug_info, repository};
use sqlx::{Error, Pool, Postgres};
use std::sync::Arc;
use uuid::Variant::Future;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    custom_log::init();

    let app_cfg = Arc::new(Config::new());
    debug_info!("main api {:?}", &app_cfg);

    let typesense_pkg = Arc::new(typesense::TypesenseImpl::new(
        app_cfg.typesense().host(),
        app_cfg.typesense().api_key(),
    ));

    let pool: Arc<Pool<Postgres>> = Arc::new(PostgresPool::new(&app_cfg).await.unwrap().pool);
    let base_repo = Arc::new(base::BaseRepositoryImpl::new(Arc::clone(&pool)));
    let cred_repo = Arc::new(credential::CredentialRepoImpl::new(Arc::clone(&pool)));
    let user_repo = Arc::new(user::UserRepoImpl::new(Arc::clone(&pool)));

    let usr_service = Arc::new(UserServiceImpl::new(user_repo.clone()));
    let auth_service = Arc::new(AuthServiceImpl::new(
        base_repo.clone(),
        cred_repo.clone(),
        user_repo.clone(),
        typesense_pkg.clone(),
    ));

    HttpServer::new(move || {
        let app_cfg = app_cfg.clone();
        let usr_service = usr_service.clone();
        let auth_service = auth_service.clone();
        App::new().configure(move |cfg| api::init(cfg, app_cfg, usr_service, auth_service))
    })
    .bind(("127.0.0.1", 8080))?
    // .workers(1) //bikin auto aja ngikutin cpu thread -> comment untuk pakei default total cpu core
    .shutdown_timeout(30)
    .run()
    .await
}
