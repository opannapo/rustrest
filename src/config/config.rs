use crate::config::database::Database;
use crate::config::typesense::Typesense;
use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    database: Database,
    typesense: Typesense,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();

        let database = Database::new();
        let typesense = Typesense::new();

        Config {
            database,
            typesense,
        }
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn typesense(&self) -> &Typesense {
        &self.typesense
    }
}
