use crate::request_dto::Cliente;
use crate::{models::ClienteModel, response_dto::ClientBalance};
use actix_web::web::Data;
use actix_web::HttpResponse;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub enum TipoTransaccion {
    Credito,
    Debito
}

pub struct AppState {
    pub clients: Arc<Mutex<HashMap<u32, ClienteModel>>>,
}
pub fn get_cliente(user_id: u32, data: &Data<AppState>, cliente: &mut ClientBalance) -> bool {
    let map = data.clients.lock().unwrap();
    match map.get(&user_id) {
        Some(cliente_encontrado) => {
            cliente.client_id = cliente_encontrado.client_id;
            cliente.client_name = cliente_encontrado.client_name.clone();
            cliente.bith_date = cliente_encontrado.bith_date;
            cliente.document_number = cliente_encontrado.document_number.clone();
            cliente.country = cliente_encontrado.country.clone();
            cliente.balance = cliente_encontrado.balance;
        }

        None => return false,
    }
    true
}
pub fn verificar_existencia_dni(    nuevo_cliente: &Cliente,
                                    clients: &mut HashMap<u32, ClienteModel>,
                                ) -> bool {
    clients.values().any(|cliente| cliente.document_number == nuevo_cliente.document_number)
}

pub fn crear_cliente(next_id: u32,  nuevo_cliente: &Cliente, clients: &mut HashMap<u32, ClienteModel>,
) {
    // Crear un nuevo ClienteModel a partir del nuevo cliente
    let cliente_model = ClienteModel {
            client_id: next_id,
            client_name: nuevo_cliente.client_name.clone(),
            bith_date: nuevo_cliente.bith_date,
            document_number: nuevo_cliente.document_number.clone(),
            country: nuevo_cliente.country.clone(),
            balance: Decimal::ZERO,
    };

    clients.insert(next_id, cliente_model);
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
