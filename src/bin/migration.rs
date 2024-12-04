use rustrest::config::config::Config;
use rustrest::debug_info;
use rustrest::util::log as custom_log;
use sqlx::{migrate::Migrator, PgPool};
use std::path::Path;

#[tokio::main]
async fn main() {
    custom_log::init();
    debug_info!("main migration");

    let cfg = Config::new();
    debug_info!("main migration {:?}", cfg);

    // postgres://username:password@host:port/database_name?sslmode=disable&timezone=Asia/Jakarta
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}?sslmode=disable&timezone=Asia/Jakarta",
        cfg.database().db_username(),
        cfg.database().db_password(),
        cfg.database().db_host(),
        cfg.database().db_port(),
        cfg.database().db_name()
    );
    debug_info!("database_url {}", database_url);

    // Buat pool koneksi
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Jalankan migrasi
    let migrator = Migrator::new(Path::new("./migration")).await.unwrap();
    match migrator.run(&pool).await {
        Ok(_) => debug_info!("Migrations ran successfully"),
        Err(e) => eprintln!("Error running migrations: {:?}", e),
    }

    drop(pool);

    println!("Migration completed and connection closed.");
}
