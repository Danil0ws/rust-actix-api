use actix_web::{get, web};

pub use crate::models::cep::CepResponse;
use crate::services::viacep::CepFactory;

#[get("/cep/{zip_code}")]
pub async fn cep_find(zip_code: web::Path<String>) -> web::Json<CepResponse> {
    let find = CepFactory::get(&zip_code.to_string()).await;
    println!("{}", &find.clone().get("cep").unwrap().to_string());
    web::Json(CepResponse {
        cep: find.clone().get("cep").unwrap().to_string(),
        bairro: find.clone().get("bairro").unwrap().to_string(),
        localidade: find
            .clone()
            .get("localidade")
            .unwrap()
            .to_owned()
            .to_string(),
        logradouro: find
            .clone()
            .get("logradouro")
            .unwrap()
            .to_owned()
            .to_string(),
        uf: find.clone().get("uf").unwrap().to_owned().to_string(),
    })
}
