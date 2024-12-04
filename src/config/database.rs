use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Database {
    db_port: i32,
    db_host: String,
    db_name: String,
    db_username: String,
    db_password: String,
}

impl Database {
    pub fn new() -> Database {
        dotenv().ok();

        let parse_int = |key: &String| -> i32 {
            let val = env::var(key).unwrap_or_else(|_| panic!("{} is not set", key));
            let res = val
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("{} is not an integer. Result is {}", key, val));
            return res;
        };

        Database {
            db_port: parse_int(&"DB_PORT".to_string()),
            db_host: env::var("DB_HOST").expect("DB_HOST is not set"),
            db_name: env::var("DB_NAME").expect("DB_NAME is not set"),
            db_username: env::var("DB_USERNAME").expect("DB_USERNAME is not set"),
            db_password: env::var("DB_PASSWORD").expect("DB_PASSWORD is not set"),
        }
    }

    pub fn db_port(&self) -> i32 {
        self.db_port
    }

    pub fn db_host(&self) -> &str {
        &self.db_host
    }

    pub fn db_name(&self) -> &str {
        &self.db_name
    }

    pub fn db_username(&self) -> &str {
        &self.db_username
    }

    pub fn db_password(&self) -> &str {
        &self.db_password
    }
}
