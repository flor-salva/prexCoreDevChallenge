use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::{request_dto::{Cliente, CreditTransaction, DebitTransaction}, response_dto::ClientBalance};
//use serde::{Serialize,Deserialize};

pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("")
        .service(client_balance);
    conf.service(scope);


}

#[get("/client_balance")]
async fn client_balance(cliente_balance: web::Json<ClientBalance>) -> impl Responder {
    
    let cliente =  ClientBalance{client_id:1, 
                                                client_name:String::from("Flor"),
                                                bith_date:NaiveDate::parse_from_str("1994-10-30", "%Y-%m-%d").expect("Error al parsear la fecha"), 
                                                document_number:String::from("38465901"), 
                                                country:String::from("Argentina"),
                                                balance:Decimal::new(6500015, 2)};
    
    //HttpResponse::Ok().body(personita.into_inner().nombre);
    HttpResponse::Ok().json(cliente)
}

#[post("/new_client")]
async fn new_client(nuevo_cliente: web::Json<Cliente>) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/new_credit_transaction")]
async fn new_credit_transaction(transaccion_credito: web::Json<CreditTransaction>) -> impl Responder {
    HttpResponse::Ok()
}
#[post("/new_debit_transaction")]
async fn new_debit_transaction(transaccion_debito: web::Json<DebitTransaction>) -> impl Responder {
    HttpResponse::Ok()
}