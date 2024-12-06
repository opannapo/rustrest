use log::error;
use rustrest::config::config::Config;
use rustrest::debug_info;
use rustrest::util::log as custom_log;
use sqlx::{migrate::Migrator, PgPool};
use std::env;
use std::fs::File;
use std::path::Path;
use std::time::SystemTime;

#[tokio::main]
async fn main() {
    custom_log::init();
    let cfg = Config::new();
    debug_info!("main migration {:?}", cfg);

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        show_help_info();
        return;
    }
    match args[1].as_str() {
        "up" => {
            run_migration_db(cfg).await;
        }
        "new" => {
            if args.len() < 3 {
                error!("Argument tidak lengkap -> migration new <file_name>");
                return;
            }
            println!("create new migration");
            create_new_sql_file(args[2].to_string());
        }
        _ => show_help_info(),
    }
}

async fn run_migration_db(cfg: Config) {
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

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let migrator = Migrator::new(Path::new("./migration")).await.unwrap();
    match migrator.run(&pool).await {
        Ok(_) => debug_info!("Migrations ran successfully"),
        Err(e) => eprintln!("Error running migrations: {:?}", e),
    }

    drop(pool);

    println!("Migration completed and connection closed.");
}

fn create_new_sql_file(arg_name: String) {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let dest = Path::new("./migration")
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let file_name = format!("{}/{:?}_{}.sql", dest, time.as_millis(), arg_name);
    println!("file name {}", file_name);

    // Membuat file kosong jika belum ada
    match File::create(file_name) {
        Ok(file) => {
            println!("File created successfully. {:?}", file);
        }
        Err(_) => {
            error!("Error creating sql file.");
        }
    }
}

fn show_help_info() {
    let run_migration = "migration up";
    let create_migration = "migration new create_table_user";

    let width1 = 25;
    let width2 = 40;

    // Menampilkan garis header dan footer
    println!();
    println!("{}", "_".repeat(width1 + width2 + 3));
    println!(
        "{:<width1$} | {:<width2$}",
        "Description",
        "Command",
        width1 = width1,
        width2 = width2
    );
    println!("{}", "_".repeat(width1 + width2 + 3));

    println!(
        "{:<width1$} | {:<width2$}",
        "Run Migration",
        run_migration,
        width1 = width1,
        width2 = width2
    );
    println!(
        "{:<width1$} | {:<width2$}",
        "Create Migration file",
        create_migration,
        width1 = width1,
        width2 = width2
    );

    println!("{}", "_".repeat(width1 + width2 + 3));
}
