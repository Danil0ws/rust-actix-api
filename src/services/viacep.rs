use std::collections::HashMap;

use reqwest::Error;
use serde_json::Value;

pub use crate::models::viacep::ViaCep;
use async_trait::async_trait;

#[async_trait]
pub trait Ceps {
    fn instance() -> Self;
    async fn request(&mut self, params: String) -> Result<HashMap<String, Value>, Error>;
}

#[async_trait]
impl Ceps for ViaCep {
    fn instance() -> Self {
        let client = reqwest::Client::new();
        Self { client }
    }
    async fn request(&mut self, params: String) -> Result<HashMap<String, Value>, Error> {
        let ip = self
            .client
            .get(format!(
                "https://viacep.com.br/ws/{}/json/",
                params.to_string()
            ))
            .send()
            .await?
            .json::<HashMap<String, Value>>()
            .await;
        return ip;
    }
}

pub struct CepFactory {}

impl CepFactory {
    pub async fn get(cep: &str) -> HashMap<String, Value> {
        let request = <ViaCep>::instance().request(cep.to_string()).await;

        match request {
            Ok(date) => date,
            Err(e) => panic!("{}", e),
        }
    }
}
