
use crate::{
    models::ClienteModel, request_dto::{Cliente, CreditTransaction, DebitTransaction}, response_dto::ClientBalance, servicios::{crear_cliente, get_cliente, verificar_existencia_dni, AppState, TipoTransaccion}
};
use actix_web::{get, post, web, HttpResponse, Responder};
use rust_decimal::Decimal;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
    .service(new_client)
    .service(new_credit_transaction)
    .service(new_debit_transaction)
    .service(client_balance);

    conf.service(scope);
}

#[get("/client_balance/{user_id}")]
async fn client_balance(path: web::Path<(u32,)>, data: web::Data<AppState>) -> impl Responder {
    let user_id = path.0;

    let mut cliente_balance = ClientBalance {
        ..Default::default()
    };

    let resultado_cliente = get_cliente(user_id, &data, &mut cliente_balance);

    if !resultado_cliente {
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

    if verificar_existencia_dni(&nuevo_cliente, &mut clients) {
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
    data: web::Data<AppState>
) -> impl Responder {
    let mut map = data.clients.lock().unwrap();
    
    if let Some(cliente_encontrado) = map.get_mut(&transaccion_credito.client_id) {
        let monto = transaccion_credito.credit_amount;
        procesar_transaccion(monto, TipoTransaccion::Credito, cliente_encontrado)
    } else {
        HttpResponse::BadRequest().body("No existe cliente con ese ID")
    }
}

#[post("/new_debit_transaction")]
async fn new_debit_transaction(transaccion_debito: web::Json<DebitTransaction>,data: web::Data<AppState>) -> impl Responder {
    
    let mut map = data.clients.lock().unwrap();
    
    if let Some(cliente_encontrado) = map.get_mut(&transaccion_debito.client_id) {
        let monto = transaccion_debito.debit_amount;
        procesar_transaccion(monto, TipoTransaccion::Debito, cliente_encontrado)
    } else {
        HttpResponse::BadRequest().body("No existe cliente con ese ID")
    }

}

pub fn procesar_transaccion(monto: Decimal, tipo_transaccion: TipoTransaccion, cliente_encontrado: &mut ClienteModel) -> HttpResponse {
    match tipo_transaccion {
        TipoTransaccion::Credito => {
            cliente_encontrado.balance += monto;
            HttpResponse::Ok().body(format!("Transacción de crédito procesada. Nuevo balance: {}", cliente_encontrado.balance))
        }
        TipoTransaccion::Debito => {
            if cliente_encontrado.balance < monto {
                return HttpResponse::BadRequest().body("Fondos insuficientes para realizar la transacción de débito");
            }
            cliente_encontrado.balance -= monto;
            HttpResponse::Ok().body(format!("Transacción de débito procesada. Nuevo balance: {}", cliente_encontrado.balance))
        }
    }
}

/*#[post("/store_balances")]
async fn store_balances(data: web::Data<AppState>) -> impl Responder {
    let mut map = data.clients.lock().unwrap();
    let store_method = FileStorage::new(FILE_PATH);

    match store_method.store_data(&map) {
        Ok(_) => {
            map.iter_mut().for_each(|(_, cliente)| {
                cliente.balance = Decimal::new(0, 0);
            });

            HttpResponse::Ok().json(SingleResponse {
                status: "éxito".to_string(),
                data: "Saldos almacenados en el archivo y reseteados correctamente en memoria".to_string(),
            })
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}*/