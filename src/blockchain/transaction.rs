use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signature: String,
    pub timestamp: i64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64, signature: String) -> Self {
        Transaction {
            from,
            to,
            amount,
            signature,
            timestamp: Utc::now().timestamp(),
        }
    }
}
