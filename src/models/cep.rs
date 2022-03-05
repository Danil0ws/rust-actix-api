use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CepResponse {
    pub cep: String,
    pub logradouro: String,
    pub bairro: String,
    pub localidade: String,
    pub uf: String,
}
