
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Default,Debug, Serialize,Deserialize)]
pub struct ClientBalance {
    pub client_id: u32,
    pub client_name: String,
    pub bith_date: NaiveDate,
    pub document_number: String,
    pub country: String,
    pub balance: Decimal,
}
