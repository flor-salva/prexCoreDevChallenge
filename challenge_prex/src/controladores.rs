use crate::{
    models::ClienteModel,
    request_dto::{Cliente, CreditTransaction, DebitTransaction},
    response_dto::ClientBalance,
    servicios::{buscar_cliente, crear_cliente, verificar_existencia_dni, AppState},
};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rust_decimal::Decimal;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("").service(new_client).service(client_balance);

    conf.service(scope);
}

#[get("/client_balance/{user_id}")]
async fn client_balance(path: web::Path<(u32,)>, data: web::Data<AppState>) -> impl Responder {
    let user_id = path.0;

    let mut cliente_balance = ClientBalance {
        ..Default::default()
    };

    let resultado_cliente = buscar_cliente(user_id, &data, &mut cliente_balance);

    if (!resultado_cliente) {
        return HttpResponse::NotFound().body("El cliente no fue encontrado");
    }

    HttpResponse::Ok().json(cliente_balance)
}

#[post("/new_client")]
async fn new_client(
    nuevo_cliente: web::Json<Cliente>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut clients = data.clients.lock().unwrap();
    if verificar_existencia_dni(&nuevo_cliente, &data, &mut clients) {
        // Si ya existe un cliente con el mismo DNI, devolver una respuesta adecuada
        return HttpResponse::BadRequest()
            .body("Ya existe un cliente con este número de documento");
    }

    let next_id = clients.len() as u32 + 1;
    crear_cliente(next_id, &nuevo_cliente, &mut clients);

    HttpResponse::Ok().body(format!("Cliente creado exitosamente con ID: {}", next_id))
}

#[post("/new_credit_transaction")]
async fn new_credit_transaction(
    transaccion_credito: web::Json<CreditTransaction>,
) -> impl Responder {
    HttpResponse::Ok()
}
#[post("/new_debit_transaction")]
async fn new_debit_transaction(transaccion_debito: web::Json<DebitTransaction>) -> impl Responder {
    HttpResponse::Ok()
}
