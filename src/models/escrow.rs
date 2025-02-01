use crate::schema::escrows;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EscrowStatus {
    Pending,
    Funded,
    Released,
    Cancelled,
}

impl EscrowStatus {
    pub fn to_string(&self) -> String {
        match self {
            EscrowStatus::Pending => "PENDING".to_string(),
            EscrowStatus::Funded => "FUNDED".to_string(),
            EscrowStatus::Released => "RELEASED".to_string(),
            EscrowStatus::Cancelled => "CANCELLED".to_string(),
        }
    }

    pub fn from_string(status: &str) -> Result<Self, String> {
        match status.to_uppercase().as_str() {
            "PENDING" => Ok(EscrowStatus::Pending),
            "FUNDED" => Ok(EscrowStatus::Funded),
            "RELEASED" => Ok(EscrowStatus::Released),
            "CANCELLED" => Ok(EscrowStatus::Cancelled),
            _ => Err("Invalid status".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = escrows)]
pub struct Escrow {
    #[diesel(skip_insertion)]
    pub id: i32,
    pub loan_amount: i64,
    pub loan_term: String,
    pub purpose_of_loan: String,
    pub monthly_income: i64,
    pub status: String,
    pub sender_address: String,
    pub recipient_address: String,
    pub locked_funds: i64,
}
