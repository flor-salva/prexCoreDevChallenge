mod request_dto;
mod response_dto;
mod controladores;
mod servicios;
mod models;

use actix_web::{ web, App, HttpServer};
//use serde::{Serialize,Deserialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(servicios::AppState::init()))
        .configure(controladores::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
