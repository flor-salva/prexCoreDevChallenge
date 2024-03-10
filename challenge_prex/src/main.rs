mod request_dto;
mod response_dto;
mod controladores;

use actix_web::{ App, HttpServer};
//use serde::{Serialize,Deserialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .configure(controladores::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
