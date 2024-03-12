mod request_dto;
mod response_dto;
mod controladores;
mod servicios;
mod models;

use std::{collections::HashMap, sync::{Arc, Mutex}};

use actix_web::{ web, App, HttpServer};
use models::ClienteModel;
use servicios::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //let app_state = Arc::new(servicios::AppState::init());
    let clients: HashMap<u32, ClienteModel> = HashMap::new();
    let state = Arc::new(Mutex::new(clients));

    HttpServer::new(move || {
        App::new()
        //.app_data(web::Data::new(servicios::AppState::init()))
        .app_data(web::Data::new(AppState { clients: state.clone() }))
        //.app_data(web::Data::new(app_state.clone()))
        .configure(controladores::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
