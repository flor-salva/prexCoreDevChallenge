use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::{request_dto::{Cliente, CreditTransaction, DebitTransaction}, response_dto::ClientBalance, servicios::{buscar_cliente, AppState}};
//use serde::{Serialize,Deserialize};

pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("")
        .service(client_balance);
    conf.service(scope);


}

#[get("/client_balance/{user_id}")]
async fn client_balance(path: web::Path<(u32,)>,data: web::Data<AppState>) -> impl Responder {
    
let user_id= path.0;

let mut cliente_balance= ClientBalance{..Default::default()};

let resultado_cliente= buscar_cliente(user_id,&data,&mut cliente_balance);

    if (!resultado_cliente) {
        return HttpResponse::NotFound().body("El cliente no fue encontrado");
    }

    HttpResponse::Ok().json(cliente_balance)
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