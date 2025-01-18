use actix_web::{App, HttpServer};
use rustrest::config::config::Config;
use rustrest::util::log as custom_log;
use rustrest::{api, debug_info};

use api::v1::credential;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    custom_log::init();
    debug_info!("main api");

    let cfg = Config::new();
    debug_info!("main api {:?}", cfg);

    HttpServer::new(|| App::new().configure(credential::init))
        .bind(("127.0.0.1", 8080))?
        .workers(1) //bikin auto aja ngikutin cpu thread
        .run()
        .await
}
