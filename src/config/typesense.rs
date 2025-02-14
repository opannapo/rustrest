use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Typesense {
    api_key: String,
    host: String,
}

impl Typesense {
    pub fn new() -> Typesense {
        dotenv().ok();

        Typesense {
            host: env::var("TYPESENSE_HOST").expect("TYPESENSE_HOST is not set"),
            api_key: env::var("TYPESENSE_API_KEY").expect("TYPESENSE_API_KEY is not set"),
        }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}
