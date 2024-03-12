use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Debug, Serialize,Deserialize)]
pub struct Cliente {
    pub client_name: String,
    pub bith_date: NaiveDate,
    pub document_number: String,
    pub country: String
}

#[derive(Debug, Serialize,Deserialize)]
pub struct CreditTransaction {
    pub client_id: u32,
    pub credit_amount: Decimal
}

#[derive(Debug, Serialize,Deserialize)]
pub struct DebitTransaction {
    pub client_id: u32,
    pub debit_amount: Decimal
}
