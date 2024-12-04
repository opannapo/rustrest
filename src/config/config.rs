use crate::config::database::Database;
use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    database: Database,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();

        let database = Database::new();

        Config { database: database }
    }

    pub fn database(&self) -> &Database {
        &self.database
    }
}
