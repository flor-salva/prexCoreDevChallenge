use actix_web::web::Data;
use actix_web::App;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;


use crate::controladores::client_balance;
use crate::{response_dto::ClientBalance, models::ClienteModel};


pub struct AppState {
    pub clients: Arc<Mutex<HashMap<u32, ClienteModel >>>,
    //type AppState = Arc<Mutex<HashMap<usize, Client>>>;
}
impl AppState {
    pub fn init() -> AppState {
        AppState {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
/* 
impl AppState {
    pub fn init() -> Arc<Mutex<AppState>> {
        Arc::new(Mutex::new(AppState {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }))
    }
}*/

//pub fn buscar_cliente(user_id: u32, data:Data<AppState>) -> u8 {
       /*let cliente =  ClientBalance{client_id: user_id, 
        client_name:String::from("Flor"),
        bith_date:NaiveDate::parse_from_str("1994-10-30", "%Y-%m-%d").expect("Error al parsear la fecha"), 
        document_number:String::from("38465901"), 
        country:String::from("Argentina"),
        balance:Decimal::new(6500015, 2)};
        let clients = data.clients.lock().unwrap();*/

       /* if let Some(client) = vec.iter_mut().find(|client| client.id.eq(&user_id)) {
            HttpResponse::Ok().json(SingleResponse {
                status: "success".to_string(),
                data: client,
            })
        } else {
            HttpResponse::Conflict().json(GenericResponse {
                status: "fail".to_string(),
                message: format!("Client with id: '{}' doesn't exist", client_id),
            })
        }*/
pub fn buscar_cliente(user_id: u32, data:&Data<AppState>,cliente:& mut ClientBalance) -> bool {
            let map = data.clients.lock().unwrap();
            match map.get(&user_id) {
                Some(cliente_encontrado) => {    cliente.client_id= cliente_encontrado.client_id;
                                                                cliente.client_name= cliente_encontrado.client_name.clone();
                                                                cliente.bith_date= cliente_encontrado.bith_date;
                                                                cliente.document_number= cliente_encontrado.document_number.clone();
                                                                cliente.country= cliente_encontrado.country.clone();
                                                                cliente.balance= cliente_encontrado.balance; 
                                                            }
                    
                None => return false,
                }
            true

}