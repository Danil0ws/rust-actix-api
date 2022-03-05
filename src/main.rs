use actix_web::{App, HttpServer};

mod routes;
mod services;
mod models;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routes::cep::cep_find))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
