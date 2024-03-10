use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Debug, Serialize,Deserialize)]
pub struct Cliente {
    client_name: String,
    bith_date: NaiveDate,
    document_number: String,
    country: String
}

#[derive(Debug, Serialize,Deserialize)]
pub struct CreditTransaction {
    client_id: u128,
    credit_amount: Decimal,
}

#[derive(Debug, Serialize,Deserialize)]
pub struct DebitTransaction {
    client_id: u128,
    debit_amount: Decimal,
}
