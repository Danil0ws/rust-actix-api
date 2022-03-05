use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct ViaCep {
    pub client: Client,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViaCepResponse {
    pub cep: String,
    pub logradouro: String,
    pub complemento: String,
    pub bairro: String,
    pub localidade:String,
    pub uf: String,
    pub ibge: i64,
    pub gia: String,
    pub ddd: String,
    pub siafi: i64
}