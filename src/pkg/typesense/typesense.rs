use async_trait::async_trait;
use log::error;
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::sync::Arc;

#[async_trait]
pub trait TypeSense: Send + Sync {
    async fn create(
        &self,
        collection: String,
        schema: Value,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub struct TypesenseImpl {
    client: Client,
    host: Arc<String>,
    api_key: Arc<String>,
}
impl TypesenseImpl {
    pub fn new(host: &str, api_key: &str) -> Self {
        TypesenseImpl {
            client: Client::new(),
            api_key: Arc::new(api_key.to_string()),
            host: Arc::new(host.to_string()),
        }
    }
}

#[async_trait]
impl TypeSense for TypesenseImpl {
    async fn create(
        &self,
        collection: String,
        schema: Value,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let url = format!("{}/collections/{}/documents", self.host, collection);

        match self
            .client
            .post(url)
            .header("X-TYPESENSE-API-KEY", self.api_key.as_str())
            .json(&schema)
            .send()
            .await
        {
            Ok(response) => {
                println!("Insert Response: {:?}", response);
                let status = response.status();
                let body = response.text().await.unwrap_or_else(|_| "Failed to read body".to_string());

                if status == reqwest::StatusCode::BAD_REQUEST {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Typesense error 400: {}", body),
                    )));
                }

                if !status.is_success() {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Typesense request failed ({}): {}", status, body),
                    )));
                }

                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}
