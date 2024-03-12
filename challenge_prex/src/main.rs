mod request_dto;
mod response_dto;
mod controladores;
mod servicios;
mod models;
mod repository;
use actix_web::{ web, App, HttpServer};
use models::ClienteModel;
use servicios::AppState;
use std::{collections::HashMap, sync::{Arc, Mutex}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let clients: HashMap<u32, ClienteModel> = HashMap::new();
    let state = Arc::new(Mutex::new(clients));

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState { clients: state.clone() }))
        .configure(controladores::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
