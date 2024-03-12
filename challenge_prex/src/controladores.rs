
use std::sync::atomic::Ordering;
use crate::{
    repository::{generate_file_name, FILE_COUNTER}, request_dto::{Cliente, CreditTransaction, DebitTransaction}, response_dto::ClientBalance, servicios::{crear_cliente, get_cliente, procesar_transaccion, verificar_existencia_dni, AppState, TipoTransaccion}
};
use actix_web::{get, post, web, HttpResponse, Responder};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
    .service(new_client)
    .service(new_credit_transaction)
    .service(new_debit_transaction)
    .service(store_balances)
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

#[post("/store_balances")]
async fn store_balances(state: web::Data<AppState>) -> impl Responder {
    // Incrementa el contador de archivo
    let file_counter = FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
   
    // Genera el nombre del archivo con el nuevo contador
    let file_name = generate_file_name(file_counter);
   
    // obtiene el estado de la aplicación
    let clients = state.clients.lock().unwrap();
   
    // Abre o crea el archivo
    let mut file = match OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open(&file_name)
    .await{
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().body("Error al crear el archivo"),
    };
   
    for (client_id, client) in clients.iter() {
        let line = format!("{:02}\t{}\n", client_id, client.balance);
        if let Err(_) = file.write_all(line.as_bytes()).await {
            return HttpResponse::InternalServerError().body("Error al escribir en el archivo");
        }
    }
   
    HttpResponse::Ok().body(format!("Saldos almacenados en el archivo: {}", file_name))
   }
   